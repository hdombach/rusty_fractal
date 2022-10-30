use crate::resources::{container::Container, resource_error::ResourceError};
use std::vec::Vec;

pub struct Scene {
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
        })
    }

    pub fn render(&self, gl: &glow::Context) {
        for object_id in &self.object_ids {
            self.container.render_object(*object_id, gl);
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        for object_id in &self.object_ids {
            self.container.get_object(*object_id).destroy(gl);
        }
    }
}
