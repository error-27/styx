use eframe::egui;

pub trait TabScreen {
    fn new() -> Self;
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}