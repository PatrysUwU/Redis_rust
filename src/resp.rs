mod lexer;

use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use anyhow::Result;
use lexer::Lexer;
#[derive(Debug)]
pub enum Value {
    SimpleString(String),
    BulkString(String),
    Array(Vec<String>),
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
        Ok(self.lexer.read_value(&mut self.stream).await?)
        // let bytes_read = self.stream.read_buf(&mut self.buffer).await?;
        // if bytes_read == 0{
        //     return Ok(None);
        // }
        // //TODO parsing
        // return Ok(Some(Value::SimpleString(String::from("PONG"))));
    }

    pub async fn write_value(&mut self, v: Value) -> Result<()> {
        self.stream.write(v.serialize().as_bytes()).await?;
        Ok(())
    }
}