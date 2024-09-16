use crate::models::message::ChatMessage;
use crate::errors::ChatError;

use mongodb::Client;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::protocol::Message, accept_async};
use futures_util::{SinkExt, StreamExt};
use log::info;

pub async fn handle_connection(stream: TcpStream, _db_client: Client) -> Result<(), ChatError> {
    let ws_stream = accept_async(stream).await?;
    info!("New WebSocket connection established");

    let (mut sender, mut receiver) = ws_stream.split();

    while let Some(msg) = receiver.next().await {
        let msg = msg.map_err(ChatError::WebSocketError)?;

        match msg {
            Message::Text(text) => {
                // Handle incoming text message
                let message: ChatMessage = serde_json::from_str(&text)
                    .map_err(|_| ChatError::InvalidMessageFormat)?;

                // Send the updated message list back to the client
                let json_response = serde_json::to_string(&message)
                    .map_err(|_| ChatError::InvalidMessageFormat)?;

                sender.send(Message::Text(json_response)).await?;
            }

            Message::Close(_) => {
                info!("WebSocket connection closed");
                break;
            }

            _ => { }
        }
    }

    Ok(())
}