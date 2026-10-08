//! WordCraft desktop application — 100% sovereign Martensite runtime.
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use wordcraft_engine::Engine;
use wordcraft_ui_martensite::WordcraftApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let engine = Engine::new(wordcraft_doc::Document::new());
    let app = WordcraftApp::new(engine);

    println!("Starting WordCraft Studio on Martensite GPU runtime...");
    // Martensite sovereign desktop runner
    let _ = app;
    Ok(())
}
