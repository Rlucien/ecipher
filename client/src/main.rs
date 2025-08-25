/**************************************************************************************************
 * @file main.rs
 * @authors Lucien
 * @brief
 * @n the main function invoking the application structure and logic.
 * @n It manages the application state, including the current page, sidebar mode,
 * @n body and footer pages, and window size.
 * @n It also handles user interactions and updates the view accordingly.
 * 
 * @version 0.1.0
 * @date 2025-06-24
 * 
 * @copyright
 * @n Copyright (c) 2021 by Loyss Studio., Division All rights reserved.
 * @n http://www.loyss.cn
 * 
**************************************************************************************************/
#![windows_subsystem = "windows"]

mod app;
mod logic;

/**************************************************************************************************
 * Import External Packages
**************************************************************************************************/
use iced::window::{icon, Settings};
use reqwest::Client;





/**************************************************************************************************
 * Import Internal Packages
**************************************************************************************************/
use shared::{KeyRequest, KeyResponse, encrypt_message, decrypt_message};


pub const WINDOW_INITIAL_WIDTH: f32 = 1200.0;
pub const WINDOW_INITIAL_HEIGHT: f32 = 600.0;
pub const WINDOW_MIN_WIDTH: f32 = 800.0;
pub const WINDOW_MIN_HEIGHT: f32 = 480.0;


/**************************************************************************************************
 * Function: main
 * Parameter:
 *    - NA: NA
 * Return:
 *    - Result: 返回应用程序的运行状态
 * Description: 程序的主函数，初始化应用程序并设置窗口属性
**************************************************************************************************/
#[tokio::main]
async fn main() -> iced::Result {
    application(App::title, App::update, App::view)
        .window(window::Settings {
            size: Size::new(WINDOW_INITIAL_WIDTH, WINDOW_INITIAL_HEIGHT),
            min_size: Some(Size::new(WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT)),
            resizable: true,
            visible: true,
            icon: window::icon::from_file_data(WINDOW_ICON, None).ok(),
            ..window::Settings::default()
        })
        .theme(App::theme)
        .centered()
        // .run_with(App::new())
        // .map_err(|e| e.into());
        .run()
}

