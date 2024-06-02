use std::fmt::Display;
use std::{fmt::Debug, str::FromStr};
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Instant;
use in_memory_db::KeyValueStore;
use tokio::net::TcpListener;

use crate::connection::handle_connection;

pub mod in_memory_db;
pub mod connection;

pub async fn start_server<T>(store: KeyValueStore<T>) -> Result<(), Box<dyn std::error::Error>>
where
    T: Send + Sync + Clone + FromStr + Debug + Display + 'static,
    T::Err: std::fmt::Debug,
{
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
