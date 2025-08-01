
mod config;
mod logic;

#[cfg(feature = "database")]
mod db;


use chrono::Local;
// Import the generated Util type from the slint UI module
use crate::ui::*;

pub mod ui {
    slint::include_modules!();
}


