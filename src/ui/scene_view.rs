use eframe::egui;
use egui::Key;
use egui::mutex::Mutex;
use std::{sync::Arc, rc::Rc};
use crate::structures::scene::Scene;

use super::{ui_traits::*, camera_view::CameraView};

pub struct SceneView {
    showing_right_panel: bool,
    scene: Arc<Mutex<Scene>>,
    _shared_state: Rc<crate::ui::shared_state::SharedState>,
    camera_view: CameraView,
}

impl SceneView {
    pub fn new(scene: Arc<Mutex<Scene>>, shared_state: Rc<crate::ui::shared_state::SharedState>) -> Self {
        Self {
            showing_right_panel: false,
            scene: scene.clone(),
            _shared_state: shared_state.clone(),
            camera_view: CameraView::new(scene),
        }
    }
}

impl UiElement for SceneView {
    fn render(&mut self, ui: &mut egui::Ui) {

        if self.showing_right_panel {
            egui::SidePanel::right("component_right_panel").show_inside(ui, |ui| {
                self.camera_view.render(ui);
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

        let resolution = glam::Vec2::new(rect.width(), rect.height());
        scene.lock().get_camera_mut().set_resolution(resolution);

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

