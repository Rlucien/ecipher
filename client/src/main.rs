#![windows_subsystem = "windows"]



use reqwest::Client;
use shared::{KeyRequest, KeyResponse, encrypt_message, decrypt_message};
use std::collections::HashMap;



#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    env_logger::Builder::default()
        .filter_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    let mut app_handler = AppHandler::new();
    app_handler.init_ui();

    let res = app_handler.run();
    res
}


pub struct AppHandler {
    display: DisplayController,
    window: Option<AppWindow>,
}


impl AppHandler {
    pub fn new() -> Self {

        Self {
            display: DisplayController::new(&data_controller),

            window: None,
        }
    }

    pub fn init_ui(&mut self) {
        let window = AppWindow::new().expect("Cannot create main window!");
        
        self.window = Some(window);
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        let window = self.window.as_ref().expect("Cannot access main window!");
        window.run()
    }
}