use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use self::get::GetRequest;
use self::set::SetRequest;

pub mod set;
pub mod get;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Set(SetRequest),
    Get(GetRequest),
}

pub type KeyValueStore = Arc<Mutex<HashMap<String, (String, Option<Instant>)>>>;