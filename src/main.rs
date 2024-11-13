#![allow(unused_imports)]

mod resp;

use std::io::{Read, Write};
use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::resp::Value;

#[tokio::main]
async fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    println!("New Connection!");
                    handle_conn(stream).await.unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_conn(stream: TcpStream) -> Result<()> {
    let mut handler = resp::RespHandler::new(stream);
    println!("Hanling connnnnn");
    loop {
        let value = if let Some(v) = handler.read_value().await? {
            //TODO COMMANDS, HANDLING RESPONSES
            v
        } else {
            println!("Connection ended");
            break;
        };
        handler.write_value(value).await?;
    }
    // let mut buffer = [0; 512];
    // loop {
    //     let bytes_read = stream.read(&mut buffer).await.unwrap();
    //     if bytes_read == 0 {
    //         break;
    //     }
    //     stream.write(format!("{:?}",buffer).as_bytes()).await.unwrap();
    // }
    Ok(())
}
