use eframe::egui;

use crate::traits::TabScreen;

pub struct Home {
    selected_port: u32
}

impl TabScreen for Home {
    fn new() -> Self {
        Self {
            selected_port: 0
        }
    }

    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
        });
    }
}