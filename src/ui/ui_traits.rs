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

pub trait UiElementParent {
    fn show_element(&mut self, element: impl UiElement);
}

impl UiElementParent for egui::Ui {
    fn show_element(&mut self, mut element: impl UiElement) {
        element.render(self);
    }
}
