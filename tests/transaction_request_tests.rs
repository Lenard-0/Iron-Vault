#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::{Arc, Mutex}, time::Duration};

    use super::*;
    use iron_vault::{connection::Response, in_memory_db::{get::{get, GetRequest}, set::{set, SetRequest}, KeyValueStore, Request}, start_server};
    use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream, time::sleep};

    #[tokio::test]
    async fn test_set_and_get() {
        let store: KeyValueStore<String> = Arc::new(Mutex::new(HashMap::new()));

        // Test setting and getting a value without TTL
        set(Arc::clone(&store), "key1".to_string(), "value1".to_string(), None).await;
        let value = get(Arc::clone(&store), "key1".to_string()).await;
        assert_eq!(value, Some("value1".to_string()));

        // Test setting and getting a value with TTL
        set(Arc::clone(&store), "key2".to_string(), "value2".to_string(), Some(2)).await;
        let value = get(Arc::clone(&store), "key2".to_string()).await;
        assert_eq!(value, Some("value2".to_string()));

        // Wait for the TTL to expire
        sleep(Duration::from_millis(3000)).await;
        let value = get(Arc::clone(&store), "key2".to_string()).await;
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_expiration() {
        let store: KeyValueStore<String> = Arc::new(Mutex::new(HashMap::new()));

        // Set a key with a TTL of 1 second
        set(Arc::clone(&store), "key3".to_string(), "value3".to_string(), Some(1)).await;

        // Ensure the value is retrievable immediately
        let value = get(Arc::clone(&store), "key3".to_string()).await;
        assert_eq!(value, Some("value3".to_string()));

        // Wait for the TTL to expire
        sleep(Duration::from_millis(2000)).await;

        // Ensure the value is no longer retrievable
        let value = get(Arc::clone(&store), "key3".to_string()).await;
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_no_expiration() {
        let store: KeyValueStore<String> = Arc::new(Mutex::new(HashMap::new()));

        // Set a key without a TTL
        set(Arc::clone(&store), "key4".to_string(), "value4".to_string(), None).await;

        // Ensure the value is retrievable
        let value = get(Arc::clone(&store), "key4".to_string()).await;
        assert_eq!(value, Some("value4".to_string()));

        // Wait for a while
        sleep(Duration::from_millis(2000)).await;

        // Ensure the value is still retrievable
        let value = get(Arc::clone(&store), "key4".to_string()).await;
        assert_eq!(value, Some("value4".to_string()));
    }

    #[tokio::test]
    async fn test_auto_removal_after_ttl() {
        let store: KeyValueStore<String> = Arc::new(Mutex::new(HashMap::new()));
        tokio::spawn(async move {
            start_server(store).await.unwrap();
        });

        // Give the server a moment to start
        sleep(Duration::from_secs(1)).await;

        let mut client = TcpStream::connect("127.0.0.1:4000").await.unwrap();

        // Test setting a key-value pair with TTL
        let set_request = Request::Set(SetRequest {
            key: "key_ttl_test".to_string(),
            value: "value_ttl_test".to_string(),
            ttl: Some(2000),
        });
        let set_request_data = serde_json::to_vec(&set_request).unwrap();
        client.write_all(&set_request_data).await.unwrap();

        let mut buffer = [0; 1024];
        let n = client.read(&mut buffer).await.unwrap();
        let response: Response = serde_json::from_slice(&buffer[..n]).unwrap();
        assert_eq!(response.status, "ok");
        assert_eq!(response.value, None);

        // Test getting the key-value pair immediately
        let get_request = Request::Get(GetRequest {
            key: "key_ttl_test".to_string(),
        });
        let get_request_data = serde_json::to_vec(&get_request).unwrap();
        client.write_all(&get_request_data).await.unwrap();

        let n = client.read(&mut buffer).await.unwrap();
        let response: Response = serde_json::from_slice(&buffer[..n]).unwrap();
        assert_eq!(response.status, "ok");
        assert_eq!(response.value, Some("value_ttl_test".to_string()));

        // Wait for the TTL to expire
        sleep(Duration::from_secs(3)).await;

        // Test getting the key-value pair after TTL expires
        let get_request = Request::Get(GetRequest {
            key: "key_ttl_test".to_string(),
        });
        let get_request_data = serde_json::to_vec(&get_request).unwrap();
        client.write_all(&get_request_data).await.unwrap();

        let n = client.read(&mut buffer).await.unwrap();
        let response: Response = serde_json::from_slice(&buffer[..n]).unwrap();
        assert_eq!(response.status, "ok");
        assert_eq!(response.value, None);
    }
}