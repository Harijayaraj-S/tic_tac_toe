// Server

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex},
};
use tokio_tungstenite::accept_async;
use tungstenite::Message;

use crate::game::Game;

type Sender = mpsc::UnboundedSender<Message>;

struct ChannelManager {
    channels: Arc<Mutex<HashMap<String, Vec<Sender>>>>,
    game: Arc<Mutex<Game>>,
}

impl ChannelManager {
    fn new() -> Self {
        ChannelManager {
            channels: Arc::new(Mutex::new(HashMap::new())),
            game: Arc::new(Mutex::new(Game::new(String::new(), String::new(), 'x'))),
        }
    }

    async fn create_channel(&self) {
        let mut channels = self.channels.lock().await;
        if !channels.contains_key("GAME") {
            channels.insert("GAME".to_string(), Vec::new());
        }
    }

    async fn add_sender_to_channel(&self, sender: Sender, user: &str) {
        let mut channels = self.channels.lock().await;
        let mut game = self.game.lock().await;

        if let Some(senders) = channels.get_mut("GAME") {
            if senders.len() > 2 {
                senders.clear();
                return;
            }

            if senders.is_empty() {
                game.player_x = user.to_string()
            } else {
                game.player_o = user.to_string();
            }

            senders.push(sender)
        }
    }

    async fn broadcast(&self, message: Message) {
        let mut channels = self.channels.lock().await;
        if let Some(senders) = channels.get_mut("GAME") {
            for sender in senders.iter() {
                sender
                    .send(message.clone())
                    .expect("Failed to send message");
            }
        }
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    channel_manager: Arc<ChannelManager>,
) -> Result<(), String> {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    println!("New WebSocket Connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel();
    let channel_manager = channel_manager.clone();

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            write.send(msg).await.expect("Failed to send message");
        }
    });

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if text.starts_with("START:") {
                    let user = &text[6..];
                    channel_manager.create_channel().await;

                    channel_manager
                        .add_sender_to_channel(tx.clone(), user)
                        .await;
                } else if text.starts_with("MOVE:") {
                    let parts: Vec<&str> = text[5..].splitn(2, ':').collect();
                    let mut game = channel_manager.game.lock().await;
                    let player = if game.player_o == parts[0] { 'o' } else { 'x' };
                    let place = parts[1].parse::<usize>().map_err(|err| err.to_string())?;

                    if let Err(err) = game.make_move(player, place) {
                        channel_manager
                            .broadcast(Message::text(format!("{}", err)))
                            .await;
                    }

                    if let Some(winner) = game.check_winner() {
                        let winner_name = if winner == 'x' {
                            game.player_x.clone()
                        } else {
                            game.player_o.clone()
                        };

                        channel_manager
                            .broadcast(Message::text(format!("The winner is {}", winner_name)))
                            .await;
                    }

                    channel_manager
                        .broadcast(Message::text(format!(
                            "{} : {} : \n{}",
                            parts[0],
                            parts[1],
                            game.print_board()
                        )))
                        .await;
                }
            }
            Ok(_) => {}
            Err(e) => eprintln!("Error receiving message: {}", e),
        }
    }

    Ok(())
}

pub async fn start_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("webSocket server Started at ws://127.0.0.1:8080");
    let channel_manager = Arc::new(ChannelManager::new());

    while let Ok((stream, addr)) = listener.accept().await {
        let channel_manager: Arc<ChannelManager> = channel_manager.clone();
        tokio::spawn(handle_connection(stream, addr, channel_manager));
    }

    Ok(())
}
