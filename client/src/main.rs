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


/**************************************************************************************************
 * Import the slint UI module
**************************************************************************************************/



/**************************************************************************************************
 * Import External Packages
**************************************************************************************************/
use reqwest::Client;
use std::collections::HashMap;

use std::rc::Rc;


/**************************************************************************************************
 * Import Internal Packages
**************************************************************************************************/
use shared::{KeyRequest, KeyResponse, encrypt_message, decrypt_message};


/**************************************************************************************************
 * Function: main
 * Parameter:
 *    - NA: NA
 * Return:
 *    - Result: 返回应用程序的运行状态
 * Description: 程序的主函数，初始化应用程序并设置窗口属性
**************************************************************************************************/
#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    
    //log::debug!("start...");

    ui_before().await;

    let mut app_handler = AppWindow::new().unwrap();

    ui_after(&app_handler);

    app_handler.run().unwrap();

    // log::debug!("exit...");
    res
}




#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
async fn ui_before() {

    // initialize the logger
    // todo

    // initialize the configuration
    // todo

    // initialize the database if the feature is enabled
    // todo
}



async fn ui_init() {

    // initialize the logger
    // todo

    // initialize the configuration
    // todo

    // initialize the database if the feature is enabled
    // todo
}



fn ui_after(ui: &AppWindow) {
    logic::init(ui);
}




