



use crate::ui::{
    AppWindow, 
    Util
};

use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_hide_window(move || {
        _ = ui_handle.unwrap().hide();
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_show_window(move || {
        _ = ui_handle.unwrap().show();
    });

    ui.global::<Util>().on_close_window(move || {
        std::process::exit(0);
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_min_window(move |minimized| {
        ui_handle.unwrap().window().set_minimized(minimized);
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>()
        .on_get_is_min_window(move || ui_handle.unwrap().window().is_minimized());

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_max_window(move |maximized| {
        ui_handle.unwrap().window().set_maximized(maximized);
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>()
        .on_get_is_max_window(move || ui_handle.unwrap().window().is_maximized());

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_fullscreen(move |fullscreen| {
        ui_handle.unwrap().window().set_fullscreen(fullscreen);
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>()
        .on_get_is_fullscreen(move || ui_handle.unwrap().window().is_fullscreen());

}