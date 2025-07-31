use eframe::{egui, epi};
use shared::{HelloRequest, HelloResponse};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Client",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {
    name: String,
    server_response: String,
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Rust Client"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello from Rust Client!");

            ui.horizontal(|ui| {
                ui.label("Enter your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            if ui.button("Send to Server").clicked() {
                let resp = send_hello(&self.name);
                self.server_response = resp;
            }

            ui.label(format!("Server response: {}", self.server_response));
        });
    }
}

fn send_hello(name: &str) -> String {
    let client = reqwest::blocking::Client::new();
    let req = HelloRequest {
        name: name.to_string(),
    };
    let url = "http://127.0.0.1:8080/hello";
    match client.post(url)
        .json(&req)
        .send()
        .and_then(|r| r.json::<HelloResponse>()) {
        Ok(resp) => resp.message,
        Err(e) => format!("Error: {}", e),
    }
}