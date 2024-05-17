use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use super::KeyValueStore;


#[derive(Serialize, Deserialize, Debug)]
pub struct SetRequest {
    pub key: String,
    pub value: String,
    pub ttl: Option<u64>,
}

pub async fn set(store: KeyValueStore, key: String, value: String, ttl: Option<u64>) {
    let expire_at = ttl.map(|t| Instant::now() + Duration::from_secs(t));
    let mut map = store.lock().unwrap();
    map.insert(key, (value, expire_at));
}