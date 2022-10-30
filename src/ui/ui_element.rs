use eframe::egui;

pub trait UiElement {
    fn render(&mut self, ui: &mut egui::Ui);
}
