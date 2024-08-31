use std::path::Path;

use eframe::egui;

pub struct Settings {
    ports: Box<Vec<SrcPort>>
}

struct SrcPort {
    name: Box<String>,
    path: Box<Path>
}

impl Settings {
    pub fn new() -> Self {
        Self { ports: Box::new(vec![]) }
    }

    pub fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.label("settings page");
        });
    }
}