use bytes::BytesMut;
use anyhow::{anyhow, Result};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use crate::resp::Value;

pub struct Lexer {
    buffer: BytesMut,
    read_position: usize,
}
impl Lexer {
    pub fn new() -> Self {
        Lexer {
            buffer: BytesMut::with_capacity(512),
            read_position: 0,
        }
    }

    async fn update_buffer(&mut self, stream: &mut TcpStream) -> Result<Option<()>> {
        println!("Updating buffer");
        self.buffer.clear();
        match stream.read_buf(&mut self.buffer).await {
            Ok(read_butes_count) => {
                if read_butes_count == 0 {
                    return Ok(None);
                }
                self.read_position = 0;
                Ok(Some(()))
            }

            Err(e) => {
                return Ok(None)
            }
        }
    }

    pub async fn read_value(&mut self, stream: &mut TcpStream) -> Result<Option<Value>> {
        if let Some(_) = self.update_buffer(stream).await? {
            if let Ok(mess) = self.consume() {
                Ok(Some(self.parse_message(&mess)?))
            } else {
                Err(anyhow::anyhow!("Invalid format {:?}",self.buffer))
            }
        } else {
            Ok(None)
        }
    }

    fn parse_message(&mut self, mess: &u8) -> Result<Value> {
        match mess {
            b'+' => self.handle_simple_string(),
            b'$' => self.handle_bulk_string(),
            b'*' => self.handle_array(),
            _ => Err(anyhow::anyhow!("Not supported type {}",mess))
        }
    }

    fn handle_simple_string(&mut self) -> Result<Value> {
        let mut result_string = String::new();
        loop {
            if self.is_crlf_next()? {
                break;
            }
            result_string.push(self.consume()?.to_owned() as char);
        }
        self.skip_crlf();
        Ok(Value::SimpleString(result_string))
    }

    fn handle_bulk_string(&mut self) -> Result<Value> {
        let mut result_string = String::new();
        let bulk_string_length: usize = self.consume_int()?;
        self.skip_crlf();
        for _ in 0..bulk_string_length {
            result_string.push(self.consume()?.to_owned() as char);
        }
        self.skip_crlf();
        Ok(Value::BulkString(result_string))
    }

    fn handle_array(&mut self) -> Result<Value> {
        let mut result_vec: Vec<Value> = Vec::new();
        let arr_length: usize = self.consume_int()?;
        self.skip_crlf();
        for _ in 0..arr_length {
            let consumed = self.consume()?;
            result_vec.push(self.parse_message(&consumed)?);
        }
        println!("{:?}", result_vec);
        Ok(Value::Array(result_vec))
    }
    fn consume_int(&mut self) -> Result<usize> {
        let mut res_string = String::new();
        loop {
            res_string.push(self.consume()?.to_owned() as char);
            if let Some(x) = self.peek() {
                if !x.is_ascii_digit() {
                    break;
                }
            } else {
                break;
            }
        }
        return self.parse_int(res_string.as_ref());
    }

    fn peek(&self) -> Option<&u8> {
        if self.is_at_end() {
            println!("End of buffer!");
            return None;
        }
        Some(&self.buffer[self.read_position])
    }
    fn parse_int(&self, number: &[u8]) -> Result<usize> {
        Ok(String::from_utf8(number.to_vec())?.parse::<usize>()?)
    }
    fn consume(&mut self) -> Result<u8> {
        if self.is_at_end() {
            return Err(anyhow!("Exceeded buffer len in consume"));
        }
        let temp = &self.buffer[self.read_position];
        self.read_position += 1;
        return Ok(temp.clone());
    }

    fn skip_crlf(&mut self) {
        self.read_position += 2;
    }

    fn is_crlf_next(&self) -> Result<bool> {
        if self.buffer[self.read_position] == b'\r' && self.buffer[self.read_position + 1] == b'\n' {
            return Ok(true);
        }
        Ok(false)
    }

    fn is_at_end(&self) -> bool {
        if self.read_position >= self.buffer.len() {
            return true;
        }
        false
    }
}