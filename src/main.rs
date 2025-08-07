// Entry point – launches the GUI.
// Starts the GUI app using eframe::run_native().

mod app;
mod network;
mod message;
mod utils;

use app::ChatApp;

fn main() -> eframe::Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Rust GUI Chat Client", options, Box::new(|_cc| Box::new(ChatApp::new())))
}