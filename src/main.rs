#![allow(unused_imports)]

use std::io::{Read, Write};
use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((mut stream,_)) => {
                tokio::spawn(async move {
                    println!("New Connection!");
                    handle_conn(stream).await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_conn(mut stream:TcpStream){

    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer).await.unwrap();
        if bytes_read == 0 {
            break;
        }
        stream.write(b"+PONG\r\n").await.unwrap();
    }
}
