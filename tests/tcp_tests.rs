#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::{Arc, Mutex}, time::Duration};

    use super::*;
    use iron_vault::{connection::Response, in_memory_db::{get::{get, GetRequest}, set::{set, SetRequest}, KeyValueStore, Request}, start_server};
    use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream, time::sleep};

    #[tokio::test]
    async fn test_tcp_set_and_get() {
        let store: KeyValueStore<String> = Arc::new(Mutex::new(HashMap::new()));
        tokio::spawn(async move {
            start_server(store).await.unwrap();
        });

        // Give the server a moment to start
        tokio::time::sleep(Duration::from_millis(1000)).await;

        let mut client = TcpStream::connect("127.0.0.1:4000").await.unwrap();

        // Test setting a key-value pair
        let set_request = Request::Set(SetRequest {
            key: "key1".to_string(),
            value: "value1".to_string(),
            ttl: None,
        });
        let set_request_data = serde_json::to_vec(&set_request).unwrap();
        client.write_all(&set_request_data).await.unwrap();

        let mut buffer = [0; 1024];
        let n = client.read(&mut buffer).await.unwrap();
        let response: Response = serde_json::from_slice(&buffer[..n]).unwrap();
        assert_eq!(response.status, "ok");
        assert_eq!(response.value, None);

        // Test getting the key-value pair
        let get_request = Request::Get(GetRequest {
            key: "key1".to_string(),
        });
        let get_request_data = serde_json::to_vec(&get_request).unwrap();
        client.write_all(&get_request_data).await.unwrap();

        let n = client.read(&mut buffer).await.unwrap();
        let response: Response = serde_json::from_slice(&buffer[..n]).unwrap();
        assert_eq!(response.status, "ok");
        assert_eq!(response.value, Some("value1".to_string()));
    }

    #[tokio::test]
    async fn test_tcp_set_with_ttl() {
        let store: KeyValueStore<String> = Arc::new(Mutex::new(HashMap::new()));
        tokio::spawn(async move {
            start_server(store).await.unwrap();
        });

        // Give the server a moment to start
        tokio::time::sleep(Duration::from_millis(1000)).await;

        let mut client = TcpStream::connect("127.0.0.1:4000").await.unwrap();

        // Test setting a key-value pair with TTL
        let set_request = Request::Set(SetRequest {
            key: "key2".to_string(),
            value: "value2".to_string(),
            ttl: Some(2),
        });
        let set_request_data = serde_json::to_vec(&set_request).unwrap();
        client.write_all(&set_request_data).await.unwrap();

        let mut buffer = [0; 1024];
        let n = client.read(&mut buffer).await.unwrap();
        let response: Response = serde_json::from_slice(&buffer[..n]).unwrap();
        assert_eq!(response.status, "ok");
        assert_eq!(response.value, None);

        // Test getting the key-value pair before TTL expires
        let get_request = Request::Get(GetRequest {
            key: "key2".to_string(),
        });
        let get_request_data = serde_json::to_vec(&get_request).unwrap();
        client.write_all(&get_request_data).await.unwrap();

        let n = client.read(&mut buffer).await.unwrap();
        let response: Response = serde_json::from_slice(&buffer[..n]).unwrap();
        assert_eq!(response.status, "ok");
        assert_eq!(response.value, Some("value2".to_string()));

        // Wait for the TTL to expire
        tokio::time::sleep(Duration::from_millis(3000)).await;

        // Test getting the key-value pair after TTL expires
        let get_request = Request::Get(GetRequest {
            key: "key2".to_string(),
        });
        let get_request_data = serde_json::to_vec(&get_request).unwrap();
        client.write_all(&get_request_data).await.unwrap();

        let n = client.read(&mut buffer).await.unwrap();
        let response: Response = serde_json::from_slice(&buffer[..n]).unwrap();
        assert_eq!(response.status, "ok");
        assert_eq!(response.value, None);
    }
}