use std::{sync::Arc, thread::sleep, time::{Duration, Instant}};

use serde::{Deserialize, Serialize};

use super::KeyValueStore;


#[derive(Serialize, Deserialize, Debug)]
pub struct SetRequest {
    pub key: String,
    pub value: String,
    pub ttl: Option<u64>,
}

pub async fn set(store: KeyValueStore, key: String, value: String, ttl: Option<u64>) {
    let expire_at = ttl.map(|t| Instant::now() + Duration::from_millis(t));
    let mut map = store.lock().unwrap();
    map.insert(key.clone(), (value, expire_at));

    // If a TTL is provided, spawn a task to remove the key after the TTL expires
    if let Some(ttl) = ttl {
        let store_clone = Arc::clone(&store);
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(ttl)).await;
            remove_key(store_clone, key).await;
        });
    }
}

async fn remove_key(store: KeyValueStore, key: String) {
    let mut map = store.lock().unwrap();
    map.remove(&key);
}