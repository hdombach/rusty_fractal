use eframe::egui;

pub trait UiElement {
    fn render(&mut self, ui: &mut egui::Ui);
}

pub trait UiScene {
    fn has_right_panel(&self) -> bool {
        false
    }
    fn right_panel_state(&mut self) -> Option<&mut bool> { None }

    fn has_left_panel(&self) -> bool {
        false
    }
    fn left_panel_state(&mut self) -> Option<&mut bool> { None }
}
