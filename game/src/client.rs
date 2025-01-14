// Client

use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tungstenite::Message;

pub async fn start_client() -> Result<()> {
    let (ws_stream, _) = connect_async("ws://127.0.0.1:8080").await?;
    println!("Connection to WebSocket server");

    let (mut write, mut read) = ws_stream.split();

    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    let message = Arc::new(Mutex::new(Vec::new()));

    tokio::spawn({
        async move {
            while let Some(msg) = rx.recv().await {
                write.send(msg).await.expect("Failed to send message");
            }
        }
    });

    tokio::spawn({
        let message = message.clone();
        async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        let mut msgs = message.lock().unwrap();
                        msgs.push(text.clone());
                        println!("{}", text);
                    }
                    Err(e) => eprintln!("Error receiving message: {}", e),
                    _ => {
                        println!("Received an unknown message type");
                    }
                }
            }
        }
    });

    print!("Enter your username: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    tx.send(Message::text(format!("START:{}", username)))
        .expect("Failed to send message");
    println!("You created and joined the game");

    loop {
        print!("{} > ", username);
        io::stdout().flush()?;
        let mut message = String::new();
        io::stdin().read_line(&mut message)?;
        let message = message.trim();

        if message == "/leave" {
            tx.send(Message::text(format!("LEAVE_GAME: {}", username)))
                .expect("Failed to send message");
            println!("You left game: {}", username);
            break;
        }

        tx.send(Message::text(format!("MOVE:{}:{}", username, message)))
            .expect("Failed to send message")
    }

    Ok(())
}
