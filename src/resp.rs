mod lexer;

use std::collections::HashMap;
use std::num::ParseIntError;
use std::ops::Add;
use std::time::Duration;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use anyhow::{anyhow, Result};
use tokio::time::Instant;
use lexer::Lexer;


#[derive(Debug, Clone)]
pub enum Value {
    SimpleString(String),
    BulkString(String),
    Array(Vec<Value>),
    NullBulkString,
}

impl Value {
    fn serialize(&self) -> String {
        match self {
            Value::SimpleString(s) => { format!("+{}\r\n", s) }
            Value::BulkString(s) => { format!("${}\r\n{}\r\n", s.len(), s) }
            Value::Array(s) => {
                let mut res = String::from(format!("*{}\r\n", s.len()));
                for val in s.iter() {
                    res.push_str(val.serialize().as_str());
                };
                res
            }
            Value::NullBulkString => { "$-1\r\n".to_string() }
            _ => panic!("Unsupported value to serialize!")
        }
    }
    fn tostring(&self) -> String {
        match self {
            Value::SimpleString(x) => { x.to_string() }
            Value::BulkString(x) => { x.to_string() }
            _ => { panic!("fsdfasfsda") }
        }
    }
}

#[derive(Debug)]
pub struct Item {
    val: Value,
    time_created: Instant,
    expiry_time: Option<Instant>,
}

impl Item {
    fn new(val: String, exp: Option<u64>) -> Self {
        let exp = match exp {
            None => {
                None
            }
            Some(x) => {
                Some(Instant::now() + Duration::from_millis(x))
            }
        };
        Item {
            val: Value::BulkString(val),
            time_created: Instant::now(),
            expiry_time: exp,
        }
    }

    fn is_expired(&self) -> bool {
        if let Some(x) = self.expiry_time {
            if Instant::now() > x {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
pub struct RespHandler {
    stream: TcpStream,
    lexer: Lexer,
    storage: HashMap<String, Item>,
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
                        Ok(Some(self.echo(args)?))
                    }
                    "SET" => {
                        match self.set(args) {
                            Ok(_) => {
                                Ok(Some(Value::SimpleString(String::from("OK"))))
                            }
                            Err(e) => {
                                Ok(Some(Value::SimpleString(String::from(format!("{}", e)))))
                            }
                        }
                    }
                    "GET" => {
                        match self.get(args) {
                            Ok(x) => {
                                Ok(x)
                            }
                            Err(e) => {
                                Ok(Some(Value::SimpleString(String::from(format!("{}", e)))))
                            }
                        }
                    }
                    "CONFIG" => {
                        match self.config(args) {
                            Ok(x) => {
                                Ok(Some(x))
                            }
                            Err(e) => {
                                Ok(Some(Value::SimpleString(String::from(format!("{}", e)))))
                            }
                        }
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

    fn config(&mut self, args: Vec<Value>) -> Result<Value> {
        let dbfilename = String::from("/tmp/redis-data");
        let dir = String::from("dump.rdb");
        if args.get(0).is_none() {
            Err(anyhow!("Not enough arguments in config"))
        } else {
            if let Value::BulkString(x) = &args[0] {
                let args = self.extract_arguments(&args);
                match args {
                    None => { Err(anyhow!("Didn't pass args to config")) }
                    Some(args) => {
                        match args.get("GET") {
                            None => { Err(anyhow!("No value for get")) }
                            Some(x) => {
                                let x = x.as_ref();
                                match x {
                                    "DIR" => {
                                        Ok(Value::Array(Vec::from([Value::BulkString("dir".to_string()), Value::BulkString(dir)])))
                                    }
                                    "DBFILENAME" => {
                                        Ok(Value::Array(Vec::from([Value::BulkString("dbfilename".to_string()), Value::BulkString(dbfilename)])))
                                    }
                                    _ => Err(anyhow!("Wrong argument in get (dbfilename,dir)"))
                                }
                            }
                        }
                    }
                }
            } else {
                Err(anyhow!("Config argument not passed as bulkstring"))
            }
        }
    }
    pub async fn write_value(&mut self, v: Value) -> Result<()> {
        self.stream.write(v.serialize().as_bytes()).await?;
        Ok(())
    }

    fn echo(&self, args: Vec<Value>) -> Result<Value> {
        if let Value::BulkString(s) = &args[0] {
            Ok(Value::BulkString(s.clone()))
        } else {
            Err(anyhow::anyhow!("No arguments provided to echo!"))
        }
    }

    fn extract_arguments(&self, args: &Vec<Value>) -> Option<HashMap<String, String>> {
        let is_odd = args.len() % 2;
        let mut result = HashMap::new();
        for i in (0..(args.len() - is_odd)).step_by(2) {
            let k = if let Value::BulkString(x) = &args[i] {
                x.to_ascii_uppercase()
            } else {
                panic!("Arg not bulkstring")
            };
            let v = if let Value::BulkString(x) = &args[i + 1] {
                x.to_ascii_uppercase()
            } else {
                panic!("Arg not bulkstring")
            };
            result.insert(k.to_string(), v.to_string());
        }
        println!("{:?}", result);
        if result.is_empty() {
            return None;
        };
        Some(result)
    }

    fn set(&mut self, args: Vec<Value>) -> Result<()> {
        if args.get(0).is_none() || args.get(1).is_none() {
            Err(anyhow!("Not enough arguments in set"))
        } else {
            if let Value::BulkString(x) = &args.clone()[0] {
                if let Value::BulkString(y) = &args.clone()[1] {
                    let args = self.extract_arguments(&args.into_iter().skip(2).collect());
                    match args {
                        Some(args) => {
                            match args.get("PX") {
                                None => {
                                    println!("no atributes item stored");
                                    self.storage.insert(x.to_string(), Item::new(y.to_string(), None));
                                }
                                Some(duration) => {
                                    let t = match duration.clone().parse::<u64>() {
                                        Ok(t) => {
                                            let test = Item::new(y.to_string(), Some(t));
                                            println!("{:?}", test);
                                            self.storage.insert(x.to_string(), test);
                                        }
                                        Err(e) => {
                                            return Err(anyhow!("{}",e));
                                        }
                                    };
                                }
                            }
                        }
                        None => {
                            self.storage.insert(x.to_string(), Item::new(y.to_string(), None));
                        }
                    }
                    println!("{:?}", self.storage);
                    Ok(())
                } else {
                    Err(anyhow!("Didn't pass enough arguments to set"))
                }
            } else {
                Err(anyhow!("Passed nonbulkstring to set"))
            }
        }
    }

    fn get(&self, args: Vec<Value>) -> Result<Option<Value>> {
        if args.get(0).is_none() {
            Err(anyhow!("Not enough arguments in get"))
        } else {
            if let Value::BulkString(x) = &args[0] {
                let item = self.storage.get(x);
                match item {
                    None => {
                        Err(anyhow!("Didn't find {} in database",x))
                    }
                    Some(x) => {
                        if !x.is_expired() {
                            Ok(Some(x.val.clone()))
                        } else {
                            Ok(Some(Value::NullBulkString))
                        }
                    }
                }
            } else {
                Err(anyhow!("Passed nonbulkstring to get"))
            }
        }
    }
}

