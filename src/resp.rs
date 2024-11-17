mod lexer;

use std::collections::HashMap;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use anyhow::{anyhow, Result};
use lexer::Lexer;
#[derive(Debug, Clone)]
pub enum Value {
    SimpleString(String),
    BulkString(String),
    Array(Vec<Value>),
}

impl Value {
    fn serialize(&self) -> String {
        match self {
            Value::SimpleString(s) => { format!("+{}\r\n", s) }
            Value::BulkString(s) => { format!("${}\r\n{}\r\n", s.len(), s) }
            Value::Array(s) => {
                format!("arejek {:?}", s)
            }
            _ => panic!("Unsupported value to serialize!")
        }
    }
    fn tostring(&self) -> String {
        match self {
            Value::SimpleString(x) => { x.to_string() }
            Value::BulkString(x) => { x.to_string() }
            Value::Array(x) => { panic!("fsdfasfsda") }
        }
    }
}
pub struct RespHandler {
    stream: TcpStream,
    lexer: Lexer,
    storage: HashMap<String, Value>,
}

impl RespHandler {
    pub fn new(stream: TcpStream) -> Self {
        RespHandler {
            stream,
            lexer: Lexer::new(),
            storage: HashMap::new(),
        }
    }

    pub async fn read_value(&mut self) -> Result<Option<Value>> {
        if let Some(v) = self.lexer.read_value(&mut self.stream).await? {
            //TODO COMMANDS, HANDLING RESPONSES
            if let Value::Array(arr) = v {
                let command = if let Some(Value::BulkString(comm)) = arr.iter().next() {
                    comm.to_ascii_uppercase()
                } else {
                    panic!("Command must be a bulk string!");
                };
                let args: Vec<Value> = arr.into_iter().skip(1).collect();
                println!("{:?}", args);
                match &command as &str {
                    "PING" => {
                        Ok(Some(Value::SimpleString(String::from("PONG"))))
                    }
                    "ECHO" => {
                        Ok(Some(self.echo(args)?))
                    }
                    "SET" => {
                        self.set(args)?;
                        Ok(Some(Value::SimpleString(String::from("OK"))))
                    }
                    "GET" => {
                        self.get(args)
                    }
                    _ => { Ok(Some(Value::SimpleString(String::from("cos nie tak kolego")))) }
                }
            } else {
                panic!("Commands have to be passed as arrays!");
            }
        } else {
            Ok(None)
        }
    }


    pub async fn write_value(&mut self, v: Value) -> Result<()> {
        self.stream.write(v.serialize().as_bytes()).await?;
        // println!("{}", v.tostring());
        // self.stream.write(v.tostring().as_bytes()).await?;
        Ok(())
    }

    fn echo(&self, args: Vec<Value>) -> Result<Value> {
        if let Value::BulkString(s) = &args[0] {
            Ok(Value::BulkString(s.clone()))
        } else {
            Err(anyhow::anyhow!("No arguments provided to echo!"))
        }
    }

    fn set(&mut self, args: Vec<Value>) -> Result<()> {
        if let Value::BulkString(x) = &args[0] {
            self.storage.insert(x.to_string(), args[1].clone());
            Ok(())
        } else {
            Err(anyhow!("Passed nonbulkstring to set"))
        }
    }

    fn get(&self, args: Vec<Value>) -> Result<Option<Value>> {
        if let Value::BulkString(x) = &args[0] {
            Ok(self.storage.get(x).cloned())
        } else {
            Err(anyhow!("Passed nonbulkstring to get"))
        }
    }
}

