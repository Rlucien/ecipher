
// mod config;
mod logic;

#[cfg(feature = "database")]
mod db;


use chrono::Local;
// Import the generated Util type from the slint UI module







// pub struct AppHandler {
//     display: DisplayController,
//     window: Option<AppWindow>,
// }


// impl AppHandler {
//     pub fn new() -> Self {

//         Self {
//             display: DisplayController::new(&data_controller),

//             window: None,
//         }
//     }

//     pub fn init_ui(&mut self) {
//         let window = AppWindow::new().expect("Cannot create main window!");
        
//         self.window = Some(window);
//     }

//     pub fn run(&self) -> Result<(), slint::PlatformError> {
//         let window = self.window.as_ref().expect("Cannot access main window!");
//         window.run()
//     }
// }