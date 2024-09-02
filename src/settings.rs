use eframe::egui;

use crate::{traits::TabScreen, AppSettings, SrcPort};

pub struct SettingsPage {
    port_name_field: String,
    port_path_field: String
}

impl TabScreen for SettingsPage {
    fn new() -> Self {
        Self {
            port_name_field: String::new(),
            port_path_field: String::new()
        }
    }

    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, settings: &mut AppSettings) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.label("Ports");
            ui.separator();
            ui.vertical(|ui| {
                for p in &settings.ports {
                    ui.label(format!("{} ({})", p.name, p.path));
                }
            });

            ui.text_edit_singleline(&mut self.port_name_field);
            ui.text_edit_singleline(&mut self.port_path_field);

            if ui.button("add").clicked() {
                let port = SrcPort {
                    name: self.port_name_field.clone(),
                    path: self.port_path_field.clone()
                };

                settings.ports.push(port);
            }
        });
    }
}