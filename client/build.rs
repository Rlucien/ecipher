
pub const VERSION: &str = env!("CARGO_PKG_VERSION");


fn main() {
    #[cfg(target_os = "windows")]
    set_windows_info();

    // _ = write_app_version();
    
    build_slint();
}

fn build_slint() {
    // Build the slint UI
    slint_build::compile("ui/app.slint").unwrap();
}

// fn write_app_version() -> Result<(), Box<dyn std::error::Error>> {
//     let tags = duct::cmd!("git", "describe", "--tags", "--abbrev=0")
//         .read()?
//         .split(char::is_whitespace)
//         .map(|s| s.to_owned())
//         .collect::<Vec<String>>();

//     let output = if let Some(version) = tags.last() {
//         format!(r#"Version: &str = "{}";"#, version)
//     } else {
//         format!(r#"Version: &str = "{}";"#, "0.0.1")
//     };

//     _ = std::fs::write("src/version.txt", output);

//     Ok(())
// }

#[cfg(target_os = "windows")]
fn set_windows_info() {
    _ = embed_resource::compile("./windows/icon.rc", embed_resource::NONE);
}