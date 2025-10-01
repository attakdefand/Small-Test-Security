// Only compile this module if websocket-testing feature is enabled
#![cfg(feature = "websocket-testing")]

#[cfg(feature = "websocket-testing")]
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[cfg(feature = "websocket-testing")]
use futures_util::{SinkExt, StreamExt};

#[cfg(feature = "websocket-testing")]
use std::time::Duration;

#[cfg(feature = "websocket-testing")]
use tokio::time::sleep;

#[cfg(feature = "websocket-testing")]
#[tokio::test]
#[ignore = "enable for WebSocket testing"]
async fn websocket_connection_test() {
    // This test requires a WebSocket server to be running
    // You would typically set this up in your environment or use a mock
    
    println!("Testing WebSocket connection...");
    
    // Example WebSocket URL - replace with your actual WebSocket endpoint
    let ws_url = std::env::var("WEBSOCKET_URL").unwrap_or("ws://localhost:8080/ws".to_string());
    
    match connect_async(&ws_url).await {
        Ok((ws_stream, _)) => {
            println!("✅ Connected to WebSocket server");
            
            let (mut write, mut read) = ws_stream.split();
            
            // Send a test message
            let test_message = Message::Text("Hello WebSocket".to_string());
            if let Err(e) = write.send(test_message).await {
                println!("❌ Failed to send message: {}", e);
                return;
            }
            
            println!("✅ Sent test message");
            
            // Wait for a response (with timeout)
            match tokio::time::timeout(Duration::from_secs(5), read.next()).await {
                Ok(Some(Ok(message))) => {
                    println!("✅ Received message: {:?}", message);
                }
                Ok(Some(Err(e))) => {
                    println!("❌ Error receiving message: {}", e);
                }
                Ok(None) => {
                    println!("❌ WebSocket connection closed");
                }
                Err(_) => {
                    println!("❌ Timeout waiting for response");
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to connect to WebSocket server: {}", e);
        }
    }
    
    println!("✅ WebSocket test completed!");
}