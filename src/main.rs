#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod app;
mod gui;
mod inputs;
mod notifications;
mod text_lib;

use app::Timer;
use std::{
    io,
    sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let app_state = Arc::new(Mutex::new(Timer::new(30)));

    gui::app_gui(app_state).expect("Failed to launch GUI");
    Ok(())
}
