use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use iron_vault::connection::handle_connection;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

type KeyValueStore = Arc<Mutex<HashMap<String, (String, Option<Instant>)>>>;

#[derive(Serialize, Deserialize, Debug)]
struct SetRequest {
    key: String,
    value: String,
    ttl: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetRequest {
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Request {
    Set(SetRequest),
    Get(GetRequest),
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    status: String,
    value: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store: KeyValueStore = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("Server listening on port 4000");

    loop {
        let (socket, _) = listener.accept().await?;
        let store = Arc::clone(&store);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, store).await {
                println!("Failed to handle connection: {:?}", e);
            }
        });
    }
}