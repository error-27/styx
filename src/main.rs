#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, App};
use home::HomePage;
use settings::SettingsPage;
use traits::TabScreen;

mod home;
mod launch;
mod settings;
mod traits;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 450.0]),
        ..Default::default()
    };
    eframe::run_native("styx", options, Box::new(|cc| Ok(Box::<Styx>::default())))
}

#[derive(PartialEq)]
enum AppTab {
    HOME,
    SETTINGS,
}

struct AppSettings {
    ports: Vec<NamedPath>,
    iwads: Vec<NamedPath>,
    pwads: Vec<NamedPath>,
}

pub struct NamedPath {
    name: String,
    path: String,
}

struct Styx {
    settings: AppSettings,
    tab: AppTab,
    home_p: HomePage,
    settings_p: SettingsPage,
}

impl Default for Styx {
    fn default() -> Self {
        Self {
            settings: AppSettings {
                ports: vec![],
                iwads: vec![],
                pwads: vec![],
            },
            tab: AppTab::HOME,
            home_p: HomePage::new(),
            settings_p: SettingsPage::new(),
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
            AppTab::HOME => self.home_p.show(ctx, frame, &mut self.settings),
            AppTab::SETTINGS => self.settings_p.show(ctx, frame, &mut self.settings),
        }
    }
}
