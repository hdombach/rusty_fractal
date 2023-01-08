use std::{sync::{Arc}, rc::Rc};

use eframe::epaint::mutex::Mutex;

use crate::structures::scene::Scene;

use super::{shared_state::SharedState, ui_traits::UiElement};


pub struct ScenePanelView {
    scene: Arc<Mutex<Scene>>,
    shared_state: Rc<SharedState>,
}

impl ScenePanelView {
    pub fn new(scene: Arc<Mutex<Scene>>, shared_state: Rc<SharedState>) -> Self {
        Self {
            scene: scene.clone(),
            shared_state: shared_state.clone(),
        }
    }
}

impl UiElement for ScenePanelView {
    fn render(&mut self, ui: &mut eframe::egui::Ui) {
        let _scene = self.scene.clone();
        ui.label("Scene");
        let list = ["one", "two", "threww", "four"];
        let mut values = [false, false, false, false];
        for (i, item) in list.iter().enumerate() {
            ui.toggle_value(&mut values[i], String::from(*item));
        }
    }
}
