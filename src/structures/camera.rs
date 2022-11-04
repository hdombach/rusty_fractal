use glam::*;

pub struct Camera {
   position: Vec3,
   rotation: Quat,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, -3.0),
            rotation: Quat::default(),
        }
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }
    pub fn set_position(&mut self, value: Vec3) {
        self.position = value;
    }
    pub fn change_position(&mut self, value: Vec3) {
        self.position += value;
    }

    pub fn get_rotation_quat(&self) -> Quat {
        self.rotation
    }
    pub fn set_rotation_quat(&mut self, value: Quat) {
        self.rotation = value;
    }
    pub fn rotate_camera(&mut self, value: Quat) {
        self.rotation *= value;
    }

    pub fn get_rotation_matrix(&self) -> Mat4 {
        Mat4::from_quat(self.rotation)
    }

    pub fn get_transformation_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(1.0, 16.0 / 9.0, 0.1, 1000.0) * Mat4::from_translation(self.position) * Mat4::from_quat(self.rotation)
    }
}
