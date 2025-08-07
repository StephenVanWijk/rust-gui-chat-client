// Async TCP networking.
// Handles async TCP connections using tokio.

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};
use std::sync::{Arc, Mutex};

pub async fn start_connection(
    address: &str,
    tx_gui: Arc<Mutex<Vec<String>>>, // Shared GUI message list
    mut rx_gui: UnboundedReceiver<String>, // From GUI to network
) {
    match TcpStream::connect(address).await {
        Ok(stream) => {
            let (read_half, mut write_half) = stream.into_split();
            let mut reader = BufReader::new(read_half).lines();

            // Spawn a task to handle incoming messages
            let incoming_tx = Arc::clone(&tx_gui);
            tokio::spawn(async move {
                while let Ok(Some(line)) = reader.next_line().await {
                    let mut messages = incoming_tx.lock().unwrap();
                    messages.push(line);
                }
            });

            // Main loop: forward GUI messages to the server
            while let Some(msg) = rx_gui.recv().await {
                let _ = write_half.write_all(format!("{}\n", msg).as_bytes()).await;
            }
        }
        Err(e) => {
            let mut messages = tx_gui.lock().unwrap();
            messages.push(format!("❌ Failed to connect: {}", e));
        }
    }
}