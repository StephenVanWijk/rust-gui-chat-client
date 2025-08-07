//  GUI state & layout (egui logic).
//  Stores chat history, input field, render logic.
//  ChatApp Struct: Holds shared state and sender
/*
    Component	                Role
    tx_gui (GUI → network)	    Sends messages typed by user
    rx_gui (used by network)	Receives those messages and writes to the TCP stream
    Arc<Mutex<Vec<String>>>	    Shared message list updated by the network and read by the GUI
 */

use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use std::sync::{Arc, Mutex};

pub struct ChatApp {
    pub messages: Arc<Mutex<Vec<String>>>,
    pub input: String,
    pub sender: Option<UnboundedSender<String>>,
}

impl ChatApp {
    pub fn new() -> Self {
        let messages = Arc::new(Mutex::new(vec![]));
        let (tx_gui, rx_gui) = unbounded_channel();

        let net_messages = Arc::clone(&messages);
        tokio::spawn(async move {
            crate::network::start_connection("127.0.0.1:8080", net_messages, rx_gui).await;
        });

        ChatApp {
            messages,
            input: String::new(),
            sender: Some(tx_gui),
        }
    }
}