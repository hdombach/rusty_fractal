use crate::egui;
use crate::structures::camera::Camera;
use crate::{Arc, Mutex};
use crate::structures::scene::Scene;

use super::common_views::{Vec3View, QuatView};
use super::ui_traits::{UiElement, UiElementParent};



pub struct CameraView {
    scene: Arc<Mutex<Scene>>,
}

impl CameraView {
    pub fn new(scene: Arc<Mutex<Scene>>) -> Self {
        Self {
            scene,
        }
    }
}

impl UiElement for CameraView {
    fn render(&mut self, ui: &mut egui::Ui) {
        Self::real_render(ui, self.scene.lock().get_camera_mut());
    }
}

impl CameraView {
    fn real_render(ui: &mut egui::Ui, camera: &mut Camera) {
        ui.show_element(Vec3View::new(camera.get_mut_position()));
        ui.separator();
        ui.show_element(QuatView::new(camera.get_mut_rotation_quat()));
    }
}
