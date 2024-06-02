use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use iron_vault::connection::handle_connection;
use iron_vault::start_server;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

type KeyValueStore = Arc<Mutex<HashMap<String, (String, Option<Instant>)>>>;

#[derive(Serialize, Debug)]
struct SetRequest<T> {
    key: String,
    value: T,
    ttl: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetRequest {
    key: String,
}

#[derive(Serialize, Debug)]
enum Request<T> {
    Set(SetRequest<T>),
    Get(GetRequest),
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    status: String,
    value: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the key-value store
    let store: KeyValueStore = Arc::new(Mutex::new(HashMap::new()));

    // Start the IronVault TCP server
    start_server(store).await?;

    Ok(())
}