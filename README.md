# Rust Tic Tac Toe (CLI)

This is a real-time multiplayer Tic Tac Toe game implemented in Rust.
It allows players to join the game over a network and play against each other via WebSockets.
The game is split into two components: a server and a client.

## Features

- Real-time multiplayer using WebSockets
- Written in Rust using `tokio` and `tokio-tungstenite`
- Simple CLI interface for starting the server or connecting as a client

## Clone the repository:

   ```bash
   git clone https://github.com/Harijayaraj-S/tic_tac_toe.git
   cd rust-tic-tac-toe/game
   ```

## Running the Game

There are two ways to run the game: as a **server** or as a **client**.

### Running the Server

To start the game as the server, run:

```bash
cargo run -- server
```

The server will listen for incoming WebSocket connections from clients.

### Running the Client

To connect as a client, run:

```bash
cargo run -- client
```

The client will attempt to connect to the server and start playing.

### Notes

- The server and client must be run on the same machine or network. Ensure that the server is started before the client.
- If you'd like to run the server and client on different machines, you'll need to modify the client to connect to the correct IP address or hostname of the server.

This readme should provide a basic overview of how to use your game and give context to the project! If you need further adjustments or additions, feel free to let me know.
