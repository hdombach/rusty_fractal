use eframe::{egui::{self, ImageButton, Modifiers, Key}, epaint::{Vec2, Color32}};
use egui::mutex::Mutex;
use std::{sync::Arc, rc::Rc};
use crate::structures::scene::Scene;

use super::ui_traits::*;

pub struct SceneView {
    pub showing_right_panel: bool,
    pub text_test: String,
    pub scene: Arc<Mutex<Scene>>,
    pub shared_state: Rc<crate::ui::shared_state::SharedState>,
}

impl SceneView {
    pub fn new(scene: Arc<Mutex<Scene>>, shared_state: Rc<crate::ui::shared_state::SharedState>) -> Self {
        Self {
            showing_right_panel: false,
            text_test: String::new(),
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
                ui.text_edit_singleline(&mut self.text_test);
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
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

        let scene = self.scene.clone();
        let drag_delta = response.drag_delta();
        let drag_delta = glam::Vec2::new(drag_delta.x * 0.004, drag_delta.y * 0.004);

        let move_delta = Self::get_move_delta(ui, 0.05);

        scene.lock().get_camera_mut().pan_camera(drag_delta);
        scene.lock().get_camera_mut().change_relative_position(move_delta);
        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                scene.lock().render(painter.gl());
            })),
        };
        ui.painter().add(callback);
    }

    fn get_move_delta(ui: &mut egui::Ui, speed: f32) -> glam::Vec3 {
        let input = ui.input_mut();
        let mut move_delta = glam::Vec3::new(0.0, 0.0, 0.0);
        if input.key_down(Key::W) {
            move_delta.z += speed;
        }
        if input.key_down(Key::S) {
            move_delta.z -= speed;
        }

        if input.key_down(Key::D) {
            move_delta.x -= speed;
        }
        if input.key_down(Key::A) {
            move_delta.x += speed;
        }

        if input.key_down(Key::E) {
            move_delta.y -= speed;
        }
        if input.key_down(Key::Q) {
            move_delta.y += speed;
        }
        
        move_delta
    }

}

