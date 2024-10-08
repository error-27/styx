use std::fmt::format;

use eframe::egui::{self, Color32, Ui};
use rfd::FileDialog;

use crate::{traits::TabScreen, AppSettings, NamedPath};

pub struct SettingsPage {
    port_name_field: String,
    port_path_field: String,
    iwad_name_field: String,
    iwad_path_field: String,
    pwad_name_field: String,
    pwad_path_field: String,
}

impl TabScreen for SettingsPage {
    fn new() -> Self {
        Self {
            port_name_field: String::new(),
            port_path_field: String::new(),
            iwad_name_field: String::new(),
            iwad_path_field: String::new(),
            pwad_name_field: String::new(),
            pwad_path_field: String::new(),
        }
    }

    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, settings: &mut AppSettings) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            // ----- SOURCE PORTS -----
            ui.label("Ports");
            ui.separator();

            path_list_viewer(ui, &mut settings.ports, "port_list");

            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.port_name_field);
            });
            ui.horizontal(|ui| {
                ui.label("Path");
                ui.text_edit_singleline(&mut self.port_path_field);
                if ui.button("Open").clicked() {
                    let file = FileDialog::new().pick_file();

                    if file.is_some() {
                        self.port_path_field = String::from(file.unwrap().to_str().unwrap());
                    }
                }
            });

            if ui.button("Add").clicked() {
                let port = NamedPath {
                    name: self.port_name_field.clone(),
                    path: self.port_path_field.clone(),
                };

                settings.ports.push(port);
                self.port_name_field = String::new();
                self.port_path_field = String::new();
            }

            ui.separator();

            // ----- IWADS -----

            ui.label("Games");
            ui.separator();

            path_list_viewer(ui, &mut settings.iwads, "iwad_list");

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

                    if file.is_some() {
                        self.iwad_path_field = String::from(file.unwrap().to_str().unwrap());
                    }
                }
            });

            if ui.button("Add").clicked() {
                let iwad = NamedPath {
                    path: self.iwad_path_field.clone(),
                    name: self.iwad_name_field.clone(),
                };

                settings.iwads.push(iwad);
                self.iwad_name_field = String::new();
                self.iwad_path_field = String::new();
            }

            ui.separator();

            // ----- PWADS -----

            ui.label("Mods");
            ui.separator();

            path_list_viewer(ui, &mut settings.pwads, "pwad_list");

            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.pwad_name_field);
            });
            ui.horizontal(|ui| {
                ui.label("Path");
                ui.text_edit_singleline(&mut self.pwad_path_field);
                if ui.button("Open").clicked() {
                    let file = FileDialog::new()
                        .add_filter("Doom Mod", &["wad", "WAD", "deh", "DEH"])
                        .pick_file();

                    if file.is_some() {
                        self.pwad_path_field = String::from(file.unwrap().to_str().unwrap());
                    }
                }
            });

            if ui.button("Add").clicked() {
                let pwad = NamedPath {
                    path: self.pwad_path_field.clone(),
                    name: self.pwad_name_field.clone(),
                };

                settings.pwads.push(pwad);
                settings.pwad_selection[0].push(settings.pwads.len() - 1);
                self.pwad_name_field = String::new();
                self.pwad_path_field = String::new();
            }
        });
    }
}

fn path_list_viewer(ui: &mut Ui, list: &mut Vec<NamedPath>, name: &str) {
    egui::ScrollArea::vertical()
        .id_source(format!("scrollplviewer{}", name))
        .max_height(100.0)
        .max_width(300.0)
        .auto_shrink(false)
        .show(ui, |ui| {
            let rect = ui.max_rect();
            if ui.is_rect_visible(rect) {
                let stroke = egui::Stroke::new(1.0, Color32::WHITE);

                ui.painter().rect_stroke(rect, 0.5, stroke);
            }
            egui::Grid::new(name)
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for (i, j) in list.clone().into_iter().enumerate() {
                        ui.label(format!("{} ({})", j.name, j.path));
                        if ui.button("Delete").clicked() {
                            list.remove(i);
                        }
                        ui.end_row();
                    }
                });
        });
}
