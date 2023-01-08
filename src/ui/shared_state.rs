use crate::resources::container::{Container, ContainerRef};
use crate::resources::object::Object;

pub struct SharedState {
    container: Container,
    selected_object: Option<ContainerRef<Object>>,
}

impl SharedState {
    pub fn new(gl: &eframe::egui::Context) -> Self {
        Self {
            container: Container::system_default(gl).unwrap(),
            selected_object: None,
        }
    }

    pub fn get_container(&self) -> &Container {
        &self.container
    }

    pub fn get_selected_object(&self) -> Option<ContainerRef<Object>> {
        if let Some(object) = self.selected_object.clone() {
            return Some(object);
        }
        return None;
    }
    pub fn set_selected_object(&mut self, name: Option<&str>) {
        if let Some(name) = name {
            self.selected_object = self.container.get_object(name);
        } else {
            self.selected_object = None;
        }
    }
}
