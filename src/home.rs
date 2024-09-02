use eframe::egui;

use crate::{traits::TabScreen, AppSettings};

pub struct HomePage {
    selected_port: u32
}

impl TabScreen for HomePage {
    fn new() -> Self {
        Self {
            selected_port: 0
        }
    }

    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, settings: &mut AppSettings) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
        });
    }
}