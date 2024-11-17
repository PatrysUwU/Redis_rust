mod lexer;

use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use anyhow::Result;
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
}
pub struct RespHandler {
    stream: TcpStream,
    lexer: Lexer,
}

impl RespHandler {
    pub fn new(stream: TcpStream) -> Self {
        RespHandler {
            stream,
            lexer: Lexer::new(),
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
                match &command as &str {
                    "PING" => {
                        Ok(Some(Value::SimpleString(String::from("PONG"))))
                    }
                    "ECHO" => {
                        Ok(Some(self.echo(args)))
                    }
                    _ => { Ok(Some(Value::SimpleString(String::from("okokoko")))) }
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
        Ok(())
    }

    fn echo(&self, args: Vec<Value>) -> Value {
        let mut res = String::new();
        for val in args {
            for c in val.serialize().chars() {
                res.push(c);
            }
        }
        Value::SimpleString(res)
    }
}

