use std::mem::size_of;

use glow::{HasContext, Context};

pub struct ShaderAttribute {
    index: Option<u32>,
    size: i32,
    data_type: u32,
    name: String,
    cpp_type: String,
}

impl ShaderAttribute {
    pub fn index(&self) -> Option<u32> {
        self.index
    }
    pub fn size(&self) -> i32 {
        self.size
    }
    pub fn data_type(&self) -> u32 {
        self.data_type
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn get_cpp_type(&self) -> &String {
        &self.cpp_type
    }

    pub fn apply_attrib(&self, gl: &glow::Context) {
        unsafe {
            if let Some(index) = self.index() {
                println!("vertex attrib, index: {}, size: {}, data_type: {}, size: {}, offset: {}", index, self.size(), self.data_type(), size_of::<f32>() as i32 * self.size(), 0);
                gl.vertex_attrib_pointer_f32(index, self.size(), self.data_type(), false, size_of::<f32>() as i32 * self.size(), 0);
            } else {
                todo!("add error logging")
            }
        }
    }

    pub fn as_vertex_shader_in(&self) -> String {
        if let Some(index) = self.index {
            format!("layout(location = {}) in {} {};\n", index, self.cpp_type, self.name)
        } else {
            todo!("error logging");
        }
    }
    pub fn as_vertex_shader_out(&self) -> String {
        format!("out {} {};\n", self.cpp_type, self.name)
    }
    pub fn as_fragment_shader_in(&self) -> String {
        format!("in {} {};\n", self.cpp_type, self.name)
    }
    pub fn as_fragment_shader_out(&self) -> String {
        format!("out {} {};\n", self.cpp_type, self.name)
    }
    pub fn as_uniform(&self) -> String {
        format!("uniform {} {};\n", self.cpp_type, self.name)
    }
}

impl ShaderAttribute {
    pub fn float4(index: u32, name: String) -> Self {
        Self {
            index: Some(index),
            size: 4,
            data_type: glow::FLOAT,
            name,
            cpp_type: String::from("vec4"),
        }
    }
    pub fn output_float4(name: String) -> Self {
        Self {
            index: None,
            size: 4,
            data_type: glow::FLOAT,
            name,
            cpp_type: String::from("vec4"),
        }
    }
    pub fn float3(index: u32, name: String) -> Self {
        Self {
            index: Some(index),
            size: 3,
            data_type: glow::FLOAT,
            name,
            cpp_type: String::from("vec3"),
        }
    }
    pub fn output_float3(name: String) -> Self {
        Self {
            index: None,
            size: 3,
            data_type: glow::FLOAT,
            name,
            cpp_type: String::from("vec3"),
        }
    }
    pub fn uniform_mat4(name: String) -> Self {
        Self {
            index: None,
            size: -1,
            data_type: 0,
            name,
            cpp_type: String::from("mat4"),
        }
    }
}
