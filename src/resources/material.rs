use glam::Vec3;
use glow::*;
use crate::resources::resource_error::ResourceError;
use crate::resources::resource_file::*;

use super::shader_attribute::ShaderAttribute;

pub enum Material {
    SolidColor(SolidColorMaterial),
}

impl Material {
    pub fn create_solid_color(color: Vec3) -> Self {
        Material::SolidColor(SolidColorMaterial::create(color))
    }
}

pub struct SolidColorMaterial {
    color: Vec3,
    color_out: ShaderAttribute,
}
impl SolidColorMaterial {
    pub fn create(color: Vec3) -> Self {
        Self {
            color,
            color_out: ShaderAttribute::output_float4(String::from("color_out")),
        }
    }
    pub fn get_color_out(&self) -> &ShaderAttribute {
        &self.color_out
    }
    pub fn get_color(&self) -> Vec3 {
        self.color
    }
}
