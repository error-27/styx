use eframe::egui;
use rfd::FileDialog;

use crate::{traits::TabScreen, AppSettings, NamedPath};

pub struct SettingsPage {
    port_name_field: String,
    port_path_field: String,
    iwad_name_field: String,
    iwad_path_field: String,
}

impl TabScreen for SettingsPage {
    fn new() -> Self {
        Self {
            port_name_field: String::new(),
            port_path_field: String::new(),
            iwad_name_field: String::new(),
            iwad_path_field: String::new(),
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

            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.port_name_field);
            });
            ui.horizontal(|ui| {
                ui.label("Path");
                ui.text_edit_singleline(&mut self.port_path_field);
                if ui.button("Open").clicked() {
                    let file = FileDialog::new().pick_file();

                    self.port_path_field = String::from(file.unwrap().to_str().unwrap());
                }
            });

            if ui.button("add").clicked() {
                let port = NamedPath {
                    name: self.port_name_field.clone(),
                    path: self.port_path_field.clone(),
                };

                settings.ports.push(port);
                self.port_name_field = String::new();
                self.port_path_field = String::new();
            }

            ui.separator();

            ui.label("Games");
            ui.separator();
            ui.vertical(|ui| {
                for i in &settings.iwads {
                    ui.label(format!("{} ({})", i.name, i.path));
                }
            });

            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.iwad_name_field);
            });
            ui.horizontal(|ui| {
                ui.label("Path");
                ui.text_edit_singleline(&mut self.iwad_path_field);
                if ui.button("Open").clicked() {
                    let file = FileDialog::new()
                        .add_filter("WAD", &["wad", "WAD"])
                        .pick_file();

                    self.iwad_path_field = String::from(file.unwrap().to_str().unwrap());
                }
            });

            if ui.button("add").clicked() {
                let iwad = NamedPath {
                    path: self.iwad_path_field.clone(),
                    name: self.iwad_name_field.clone(),
                };

                settings.iwads.push(iwad);
                self.iwad_name_field = String::new();
                self.iwad_path_field = String::new();
            }
        });
    }
}
