use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};

#[tokio::test]
#[ignore = "enable when WebSocket endpoint is available"]
async fn websocket_connection_test() {
    let ws_url = std::env::var("WS_URL").unwrap_or("ws://localhost:8080/ws".into());
    
    let (mut ws_stream, _) = connect_async(&ws_url)
        .await
        .expect("Failed to connect to WebSocket");
    
    // Send a test message
    ws_stream
        .send(Message::Text("Hello WebSocket".into()))
        .await
        .expect("Failed to send message");
    
    // Wait for a response
    if let Some(msg) = ws_stream.next().await {
        let msg = msg.expect("Failed to get message");
        assert!(!msg.to_string().is_empty());
    }
    
    // Close connection
    ws_stream.close().await.expect("Failed to close WebSocket");
}