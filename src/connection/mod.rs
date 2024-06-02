use std::{fmt::{Debug, Display}, str::FromStr};

use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use crate::in_memory_db::{
    get::{get, GetRequest},
    set::{set, SetRequest},
    KeyValueStore, Request,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub status: String,
    pub value: Option<String>,
}

pub async fn handle_connection<T>(mut socket: TcpStream, store: KeyValueStore<T>) -> Result<(), Box<dyn std::error::Error>>
where
    T: Send + Sync + Clone + FromStr + Debug + Display + 'static,
    T::Err: std::fmt::Debug,
{
    let mut buffer = [0; 1024];

    loop {
        let n = socket.read(&mut buffer).await?;

        if n == 0 {
            break;
        }

        let request: Request = serde_json::from_slice(&buffer[..n])?;

        match request {
            Request::Set(req) => {
                set(store.clone(), req.key, req.value, req.ttl).await;
                let response = Response {
                    status: "ok".to_string(),
                    value: None,
                };
                let response_data = serde_json::to_vec(&response)?;
                socket.write_all(&response_data).await?;
            }
            Request::Get(req) => {
                let value = get(store.clone(), req.key).await.map(|v| v.to_string());
                let response = Response {
                    status: "ok".to_string(),
                    value,
                };
                let response_data = serde_json::to_vec(&response)?;
                socket.write_all(&response_data).await?;
            }
        }
    }

    Ok(())
}
