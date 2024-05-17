use std::time::Instant;

use serde::{Deserialize, Serialize};

use super::KeyValueStore;



#[derive(Serialize, Deserialize, Debug)]
pub struct GetRequest {
    pub key: String,
}

pub async fn get(store: KeyValueStore, key: String) -> Option<String> {
    let mut map = store.lock().unwrap();
    if let Some((value, expire_at)) = map.get(&key) {
        if let Some(expire_at) = expire_at {
            if Instant::now() > *expire_at {
                map.remove(&key);
                return None;
            }
        }
        return Some(value.clone());
    }
    None
}