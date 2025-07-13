// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log::info;
use log4rs;
use std::path::PathBuf;

fn main() {
    init_logger().expect("Failed to initialize logger");
    info!("Application started");
    es_client_lib::run()
}

fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("log4rs.yaml");
    
    log4rs::init_file(config_path, Default::default())?;
    Ok(())
}
