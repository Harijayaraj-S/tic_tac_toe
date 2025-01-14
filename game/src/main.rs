// Main

use std::env;

mod client;
pub mod game;
mod server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "server" => server::start_server().await.unwrap(),
        "client" => client::start_client().await.unwrap(),
        _ => print!("Invaid arg"),
    }
}
