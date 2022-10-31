use glam::{Quat, Vec3, Vec3Swizzles};

use crate::{resources::{container::Container, resource_error::ResourceError}, main};
use std::vec::Vec;

use super::camera::Camera;

pub struct Scene {
    main_camera: Camera,
    current_rotation_dir: Vec3,
    container: Container,
    object_ids: Vec<usize>,
}

impl Scene {
    pub fn default(gl: &glow::Context) -> Result<Self, ResourceError> {
        let container = match Container::default(gl) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };
        let mut object_ids = Vec::new();
        object_ids.push(container.get_object_id(String::from("default")).unwrap());
        Ok(Self {
            container,
            object_ids,
            main_camera: Camera::new(),
            current_rotation_dir: Vec3::new(0.0, 0.0, 1.0),
        })
    }

    pub fn get_camera(&self) -> &Camera {
        &self.main_camera
    }

    pub fn get_container(&self) -> &Container {
        &self.container
    }

    pub fn render(&mut self, gl: &glow::Context) {
        self.current_rotation_dir = Quat::from_axis_angle(Vec3::new(1.0, 1.0, 0.0), 0.02).normalize() * self.current_rotation_dir;
        self.main_camera.rotate_camera(Quat::from_axis_angle(self.current_rotation_dir, 0.04).normalize());
        for object_id in &self.object_ids {
            self.container.render_object(*object_id, gl, self.get_camera());
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        for object_id in &self.object_ids {
            self.container.get_object(*object_id).destroy(gl);
        }
    }
}
