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

    pub fn change_relative_position(&mut self, offset: Vec3) {
        let offset = glam::Vec4::new(offset.x, offset.y, offset.z, 1.0);
        let offset = self.get_rotation_matrix().inverse() * offset;
        self.change_position(offset.xyz());
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

    pub fn pan_camera(&mut self, offset: Vec2) {
        let rotation = Quat::from_axis_angle(self.get_side_vec(), offset.y) *
            Quat::from_axis_angle(self.get_up_vec(), offset.x);
        self.rotate_camera(rotation.normalize());
    }

    pub fn get_rotation_matrix(&self) -> Mat4 {
        Mat4::from_quat(self.rotation)
    }

    pub fn get_transformation_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(1.0, 16.0 / 9.0, 0.1, 1000.0) * Mat4::from_quat(self.rotation) * Mat4::from_translation(self.position)
    }

    fn get_forward_vec(&self) -> Vec3 {
        (self.get_rotation_matrix().inverse() * Vec4::new(0.0, 0.0, 1.0, 1.0)).xyz()
    }

    fn get_side_vec(&self) -> Vec3 {
        //self.get_forward_vec().cross(Vec3::new(0.0, 1.0, 0.0))
        (self.get_rotation_matrix().inverse() * Vec4::X).xyz()
    }

    fn get_up_vec(&self) -> Vec3 {
        //self.get_forward_vec().cross(self.get_side_vec())
        (self.get_rotation_matrix().inverse() * Vec4::Y).xyz()
    }
}
