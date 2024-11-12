use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use anyhow::Result;
pub enum Value{
    SimpleString(String),
    BulkString(String),
    Array(Vec<String>)
}

impl Value{
    fn serialize(&self) -> String{
        match self {
            Value::SimpleString(s) => {format!("+{}\r\n",s)},
            Value::BulkString(s) => {format!("${}\r\n{}\r\n",s.len(),s)},
            _ => panic!("Unsupported value to serialize!")
        }
    }
}
pub struct RespHandler{
    buffer: BytesMut,
    stream: TcpStream,
    read_position: usize
}

impl RespHandler{
    pub fn new(mut stream: TcpStream) -> Self{
        RespHandler{
            buffer:BytesMut::with_capacity(512),
            stream,
            read_position:0
        }
    }

    pub async fn read_value(&mut self) -> Result<Option<Value>>{
        let bytes_read = self.stream.read_buf(&mut self.buffer).await?;
        if bytes_read == 0{
            return Ok(None);
        }
        //TODO parsing
        return Ok(Some(Value::SimpleString(String::from("PONG"))));
    }

    pub async fn write_value(&mut self, v:Value) -> Result<()>{
        self.stream.write(v.serialize().as_bytes()).await?;
        Ok(())
    }
}