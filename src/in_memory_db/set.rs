use std::{fmt::Debug, str::FromStr, sync::Arc, time::{Duration, Instant}};
use serde::{Deserialize, Serialize};
use super::KeyValueStore;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetRequest {
    pub key: String,
    pub value: String,
    pub ttl: Option<u64>,
}

pub async fn set<T>(store: KeyValueStore<T>, key: String, value: String, ttl: Option<u64>)
where
    T: Send + Sync + Clone + FromStr + Debug + 'static,
    T::Err: std::fmt::Debug, // Ensure the error type of FromStr implements Debug
{
    let parsed_value = value.parse::<T>().expect("Failed to parse value");
    let expire_at = ttl.map(|t| Instant::now() + Duration::from_millis(t));
    let mut map = store.lock().unwrap();
    map.insert(key.clone(), (parsed_value, expire_at));

    // If a TTL is provided, spawn a task to remove the key after the TTL expires
    if let Some(ttl) = ttl {
        let store_clone = Arc::clone(&store);
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(ttl)).await;
            remove_key(store_clone, key).await;
        });
    }
}

async fn remove_key<T>(store: KeyValueStore<T>, key: String)
where
    T: Send + Sync + 'static, // Ensure T is Send and Sync
{
    let mut map = store.lock().unwrap();
    map.remove(&key);
}
