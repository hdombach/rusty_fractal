use glam::Vec3;

use super::shader_attribute::ShaderAttribute;

pub struct Material {
    material_type: MaterialType,
    name: String,
}

impl Material {
    pub fn create_with_type(material_type: MaterialType, name: &str) -> Self {
        Self {
            material_type,
            name: String::from(name),
        }
    }
    pub fn create_solid_color(color: Vec3, name: &str) -> Self {
        Self::create_with_type(MaterialType::SolidColor(SolidColorMaterial::create(color)), name)
    }
    pub fn get_material_type(&self) -> &MaterialType {
        &self.material_type
    }
    pub fn get_name(&self) -> &str { &self.name }
}

pub enum MaterialType {
    SolidColor(SolidColorMaterial),
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
