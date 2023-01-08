use glam::Vec3;
use glow::HasContext;

use crate::{resources::{container::{Container, ContainerRef}, object::Object}, util::error::Error};
use std::vec::Vec;

use super::camera::Camera;

pub struct Scene {
    main_camera: Camera,
    _current_rotation_dir: Vec3,
    container: Container,
    objects: Vec<ContainerRef<Object>>,
}

impl Scene {
    pub fn default(gl: &glow::Context) -> Result<Self, Error> {
        let container = match Container::default(gl) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };
        let mut objects = Vec::new();
        objects.push(container.get_object("gargoyle").unwrap());
        objects.push(container.get_object("monkey").unwrap());
        //objects.push(container.get_object("cube").unwrap());
        Ok(Self {
            container,
            objects,
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
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.clear(glow::DEPTH_BUFFER_BIT);
        }
        for object in &self.objects {
            object.lock().unwrap().render(gl, self.get_camera());
        }
    }
}
