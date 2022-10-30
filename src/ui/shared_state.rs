use crate::resources::container::Container;

pub struct SharedState {
    container: Container,
}

impl SharedState {
    pub fn new(gl: &eframe::egui::Context) -> Self {
        Self {
            container: Container::system_default(gl).unwrap()
        }
    }

    pub fn get_container(&self) -> &Container {
        &self.container
    }
}
