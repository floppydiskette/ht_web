use futures_util::{SinkExt, StreamExt};
use warp::ws::WebSocket;
use crate::synch;

pub async fn websocket(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();
    // if we received "time", send the HDATE data along with TIME_OF_PACKET_SENT_MS in json
    match rx.next().await {
        Some(Ok(msg)) => {
            match msg.to_str().unwrap_or("") {
                "time" => {
                    let json = serde_json::to_string(&synch::HDATE.lock().unwrap().clone()).unwrap();
                    tx.send(warp::ws::Message::text(json)).await.unwrap();
                }
                _ => {
                    tx.send(warp::ws::Message::text("unknown command")).await.unwrap();
                }
            }
        }
        _ => {}
    }
}