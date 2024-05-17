#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::{Arc, Mutex}, time::Duration};

    use super::*;
    use iron_vault::in_memory_db::{get::get, set::set, KeyValueStore};
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_set_and_get() {
        let store: KeyValueStore = Arc::new(Mutex::new(HashMap::new()));

        // Test setting and getting a value without TTL
        set(Arc::clone(&store), "key1".to_string(), "value1".to_string(), None).await;
        let value = get(Arc::clone(&store), "key1".to_string()).await;
        assert_eq!(value, Some("value1".to_string()));

        // Test setting and getting a value with TTL
        set(Arc::clone(&store), "key2".to_string(), "value2".to_string(), Some(2)).await;
        let value = get(Arc::clone(&store), "key2".to_string()).await;
        assert_eq!(value, Some("value2".to_string()));

        // Wait for the TTL to expire
        sleep(Duration::from_secs(3)).await;
        let value = get(Arc::clone(&store), "key2".to_string()).await;
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_expiration() {
        let store: KeyValueStore = Arc::new(Mutex::new(HashMap::new()));

        // Set a key with a TTL of 1 second
        set(Arc::clone(&store), "key3".to_string(), "value3".to_string(), Some(1)).await;

        // Ensure the value is retrievable immediately
        let value = get(Arc::clone(&store), "key3".to_string()).await;
        assert_eq!(value, Some("value3".to_string()));

        // Wait for the TTL to expire
        sleep(Duration::from_secs(2)).await;

        // Ensure the value is no longer retrievable
        let value = get(Arc::clone(&store), "key3".to_string()).await;
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_no_expiration() {
        let store: KeyValueStore = Arc::new(Mutex::new(HashMap::new()));

        // Set a key without a TTL
        set(Arc::clone(&store), "key4".to_string(), "value4".to_string(), None).await;

        // Ensure the value is retrievable
        let value = get(Arc::clone(&store), "key4".to_string()).await;
        assert_eq!(value, Some("value4".to_string()));

        // Wait for a while
        sleep(Duration::from_secs(2)).await;

        // Ensure the value is still retrievable
        let value = get(Arc::clone(&store), "key4".to_string()).await;
        assert_eq!(value, Some("value4".to_string()));
    }
}