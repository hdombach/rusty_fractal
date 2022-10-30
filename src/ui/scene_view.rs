use eframe::{egui::{self, ImageButton}, epaint::{Vec2, Color32}};
use egui::mutex::Mutex;
use std::{sync::Arc, rc::Rc};
use crate::structures::scene::Scene;

use super::ui_traits::*;

pub struct SceneView {
    pub showing_right_panel: bool,
    pub scene: Arc<Mutex<Scene>>,
    pub shared_state: Rc<crate::ui::shared_state::SharedState>,
}

impl SceneView {
    pub fn new(scene: Arc<Mutex<Scene>>, shared_state: Rc<crate::ui::shared_state::SharedState>) -> Self {
        Self {
            showing_right_panel: false,
            scene,
            shared_state,
        }
    }
}

impl UiElement for SceneView {
    fn render(&mut self, ui: &mut egui::Ui) {

        if self.showing_right_panel {
            egui::SidePanel::right("component_right_panel").show_inside(ui, |ui| {
                ui.label("right panel");
            });
        }
        self.scene_viewport(ui);
    }
}

impl UiScene for SceneView {
    fn has_right_panel(&self) -> bool {
        true
    }
    fn right_panel_state(&mut self) -> Option<&mut bool> {
        Some(&mut self.showing_right_panel)
    }
}

impl SceneView {
    fn scene_viewport(&mut self, ui: &mut egui::Ui) {
        let (rect, _) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

        let scene = self.scene.clone();
        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                scene.lock().render(painter.gl());
            })),
        };
        ui.painter().add(callback);
    }
}

