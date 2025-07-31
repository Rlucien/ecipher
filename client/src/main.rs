use eframe::{egui, epi};
use shared::{encrypt_message, decrypt_message, KeyRequest, KeyResponse};
use reqwest::blocking::Client;

#[derive(Default)]
struct MyApp {
    key_id: String,
    plaintext: String,
    ciphertext: String,
    decrypted: String,
    key: Vec<u8>,
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Encrypt Tool with Remote Key"
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.heading("Header - Rust Encrypt Client");
        });
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.label("Footer © 2025 Rust Encrypt Demo");
        });
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            ui.label("Sidebar");
            if ui.button("获取密钥").clicked() {
                let client = Client::new();
                let resp = client.post("http://127.0.0.1:8080/get_key")
                    .json(&KeyRequest { key_id: self.key_id.clone() })
                    .send()
                    .and_then(|r| r.json::<KeyResponse>());
                if let Ok(r) = resp {
                    self.key = r.key;
                }
            }
            if ui.button("新建密钥").clicked() {
                let client = Client::new();
                let resp = client.post("http://127.0.0.1:8080/store_key")
                    .json(&KeyRequest { key_id: self.key_id.clone() })
                    .send()
                    .and_then(|r| r.json::<KeyResponse>());
                if let Ok(r) = resp {
                    self.key = r.key;
                }
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("密钥ID:");
                ui.text_edit_singleline(&mut self.key_id);
            });
            ui.horizontal(|ui| {
                ui.label("原文:");
                ui.text_edit_singleline(&mut self.plaintext);
            });
            if ui.button("加密").clicked() {
                if !self.key.is_empty() {
                    if let Some(ct) = encrypt_message(&self.key, &self.plaintext) {
                        self.ciphertext = base64::encode(ct);
                    }
                }
            }
            ui.horizontal(|ui| {
                ui.label("密文:");
                ui.text_edit_singleline(&mut self.ciphertext);
            });
            if ui.button("解密").clicked() {
                if !self.key.is_empty() {
                    if let Ok(ct) = base64::decode(&self.ciphertext) {
                        if let Some(pt) = decrypt_message(&self.key, &ct) {
                            self.decrypted = pt;
                        }
                    }
                }
            }
            ui.horizontal(|ui| {
                ui.label("解密结果:");
                ui.text_edit_singleline(&mut self.decrypted);
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Encrypt Client",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}