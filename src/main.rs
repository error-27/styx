#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use home::Home;
use settings::Settings;
use traits::TabScreen;

mod home;
mod settings;
mod launch;
mod traits;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "styx",
        options,
        Box::new(|cc| {
            Ok(Box::<Styx>::default())
        })
    )
}

#[derive(PartialEq)]
enum AppTab {
    HOME,
    SETTINGS
}

struct Styx {
    tab: AppTab,
    home: Home,
    settings: Settings
}

impl Default for Styx {
    fn default() -> Self {
        Self {
            tab: AppTab::HOME,
            home: Home::new(),
            settings: Settings::new()
        }
    }
}

impl eframe::App for Styx {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // render tab bar
        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, AppTab::HOME, "HOME");
                ui.selectable_value(&mut self.tab, AppTab::SETTINGS, "SETTINGS");
            });
        });

        // render corresponding tab screen
        match self.tab {
            AppTab::HOME => {
                self.home.show(ctx, frame)
            },
            AppTab::SETTINGS => {
                self.settings.show(ctx, frame)
            }
        }
    }
}
