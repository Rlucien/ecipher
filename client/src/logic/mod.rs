
pub mod util;





pub fn init(ui: &AppWindow) {
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    {
        util::init(ui);
    }
}


