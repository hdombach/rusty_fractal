use glam::Vec3;

use crate::{resources::container::Container, util::error::Error};
use std::vec::Vec;

use super::camera::Camera;

pub struct Scene {
    main_camera: Camera,
    _current_rotation_dir: Vec3,
    container: Container,
    object_ids: Vec<usize>,
}

impl Scene {
    pub fn default(gl: &glow::Context) -> Result<Self, Error> {
        let container = match Container::default(gl) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };
        let mut object_ids = Vec::new();
        object_ids.push(container.get_object_id(String::from("default")).unwrap());
        //object_ids.push(container.get_object_id(String::from("cube_mesh")).unwrap());
        Ok(Self {
            container,
            object_ids,
            main_camera: Camera::new(),
            _current_rotation_dir: Vec3::new(0.0, 0.0, 1.0),
        })
    }

    pub fn get_camera<'a>(&'a self) -> &'a Camera {
        &self.main_camera
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.main_camera
    }

    pub fn get_container(&self) -> &Container {
        &self.container
    }

    pub fn render(&mut self, gl: &glow::Context) {
        //self.current_rotation_dir = Quat::from_axis_angle(Vec3::new(1.0, 1.0, 0.0), 0.01).normalize() * self.current_rotation_dir;
        //self.main_camera.rotate_camera(Quat::from_axis_angle(self.current_rotation_dir, 0.02).normalize());
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
