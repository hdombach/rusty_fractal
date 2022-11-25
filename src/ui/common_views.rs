use std::f64::consts::PI;

use eframe::egui::DragValue;
use glam::Vec3;
use glam::Quat;
use crate::{egui, util::util::GetSetValue};

use super::ui_traits::UiElement;
use super::ui_traits::UiElementParent;

pub struct Vec3View<'a> {
    value: GetSetValue<'a, Vec3>,
    speed: f32,
    label: String,
}

impl<'a> UiElement for Vec3View<'a> {
    fn render(&mut self, ui: &mut egui::Ui) {
        let mut old_vec = (self.value)(None);

        ui.vertical(|ui| {
            ui.label(&self.label);

            let x_get_set = |v: Option<f64>| {
                if let Some(v) = v {
                    old_vec.x = v as f32;
                    (self.value)(Some(old_vec));
                }
                (self.value)(None).x as f64
            };
            ui.horizontal(|ui| {
                ui.label("x: ");
                ui.add(DragValue::from_get_set(x_get_set));
            });

            let y_get_set = |v: Option<f64>| {
                if let Some(v) = v {
                    old_vec.y = v as f32;
                    (self.value)(Some(old_vec));
                }
                (self.value)(None).y as f64
            };
            ui.horizontal(|ui| {
                ui.label("y: ");
                ui.add(DragValue::from_get_set(y_get_set));
            });
            
            let z_get_set = |v: Option<f64>| {
                if let Some(v) = v {
                    old_vec.z = v as f32;
                    (self.value)(Some(old_vec));
                }
                (self.value)(None).z as f64
            };
            ui.horizontal(|ui| {
                ui.label("z: ");
                ui.add(DragValue::from_get_set(z_get_set));
            });
        });
    }
}

impl<'a> Vec3View<'a> {
    pub fn new(value: &'a mut Vec3) -> Self {
        let get_set: GetSetValue<Vec3> = Box::new(|v: Option<Vec3>| {
            if let Some(v) = v {
                *value = v;
            }
            *value
        });
        Self::from_get_set(get_set)
    }

    pub fn from_get_set(value: GetSetValue<'a, Vec3>) -> Self {
        Self {
            value,
            speed: 0.1,
            label: String::from("position: "),
        }
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
    pub fn label(mut self, label: String) -> Self {
        self.label = label;
        self
    }
}

pub struct QuatView<'a> {
    value: GetSetValue<'a, Quat>,
    rotation_speed: f32,
    axis_speed: f32,
    label: String,
}

impl<'a> UiElement for QuatView<'a> {
    fn render(&mut self, ui: &mut egui::Ui) {
        let (axis, angle) = (self.value)(None).to_axis_angle();

        ui.vertical(|ui| {
            let axis_get_set = Box::new(|v: Option<Vec3>| {
                if let Some(v) = v {
                    (self.value)(Some(Quat::from_axis_angle(v, angle)));
                    v
                } else {
                    axis
                }
            });
            ui.label(&self.label);
            ui.horizontal(|ui| {
                ui.show_element(Vec3View::from_get_set(axis_get_set).label(String::from("axis: ")).speed(self.axis_speed));
            });

            let angle_get_set = Box::new(|v: Option<f64>| -> f64 {
                if let Some(mut v) = v {
                    if v >= PI * 2.0 {
                        v -= PI * 2.0;
                    }
                    if v < 0.0 {
                        v += PI * 2.0;
                    }
                    (self.value)(Some(Quat::from_axis_angle(axis, v as f32)));
                    v
                } else {
                    angle as f64
                }
            });
            ui.horizontal(|ui| {
                ui.label("Θ: ");
                ui.add(DragValue::from_get_set(angle_get_set).speed(self.rotation_speed));
            });
        });
    }
}

impl<'a> QuatView<'a> {
    pub fn new(value: &'a mut Quat) -> Self {
        let get_set: GetSetValue<Quat> = Box::new(|v: Option<Quat>| {
            if let Some(v) = v {
                *value = v;
            }
            *value
        });
        Self::from_get_set(get_set)
    }

    pub fn from_get_set(value: GetSetValue<'a, Quat>) -> Self {
        Self {
            value,
            label: String::from("rotation: "),
            rotation_speed: 0.02,
            axis_speed: 0.1,
        }
    }

    pub fn rotation_speed(mut self, rotation_speed: f32) -> Self {
        self.rotation_speed = rotation_speed;
        self
    }

    pub fn axis_speed(mut self, axis_speed: f32) -> Self {
        self.axis_speed = axis_speed;
        self
    }
}
