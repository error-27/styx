use eframe::egui;

use crate::{launch::launch_port, traits::TabScreen, AppSettings};

pub struct HomePage {
    selected_port: usize,
    selected_iwad: usize,
}

impl TabScreen for HomePage {
    fn new() -> Self {
        Self {
            selected_port: 0,
            selected_iwad: 0,
        }
    }

    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, settings: &mut AppSettings) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Selected Port:")
                .selected_text({
                    if settings.ports.len() > 0 {
                        settings.ports[self.selected_port].name.clone()
                    } else {
                        "None".to_string()
                    }
                })
                .show_ui(ui, |ui| {
                    for (i, p) in settings.ports.iter().enumerate() {
                        ui.selectable_value(&mut self.selected_port, i, p.name.clone());
                    }
                });

            egui::ComboBox::from_label("Selected Game:")
                .selected_text({
                    if settings.iwads.len() > 0 {
                        settings.iwads[self.selected_iwad].name.clone()
                    } else {
                        "None".to_string()
                    }
                })
                .show_ui(ui, |ui| {
                    for (i, p) in settings.iwads.iter().enumerate() {
                        ui.selectable_value(&mut self.selected_iwad, i, p.name.clone());
                    }
                });

            if ui.button("Play").clicked() {
                launch_port(
                    settings.ports[self.selected_port].path.clone(),
                    settings.iwads[self.selected_iwad].path.clone(),
                );
            }
        });
    }
}
