use std::mem;

use glow::*;
use crate::{resources::resource_error::ResourceError, structures::camera::Camera};

use super::shader_attribute::ShaderAttribute;

pub enum VertexShader {
    Simple(SimpleVertexShader),
}
impl VertexShader {
    pub fn default_simple() -> Self {
        Self::Simple(SimpleVertexShader::default())
    }
    pub fn default_simple_with_normal() -> Self {
        let properties = vec![
            (ShaderAttribute::float3(1, String::from("normal_in")), ShaderAttribute::output_float3(String::from("normal"))),
        ];
        Self::Simple(SimpleVertexShader::from_vertex_map(properties))
    }
    pub fn simple(properties: Vec<(ShaderAttribute, ShaderAttribute)>) -> Self {
        Self::Simple(SimpleVertexShader::from_vertex_map(properties))
    }
    pub fn get_vertex_out(&self) -> &ShaderAttribute {
        match self {
            Self::Simple(simple_shader) => return &simple_shader.vertex_out,
        }
    }
    pub fn apply_attributes(&self, gl: &glow::Context) {
        match self {
            Self::Simple(simple_shader) => simple_shader.apply_attributes(gl),
        }
    }
}

pub struct SimpleVertexShader {
    vertex_in: ShaderAttribute,
    vertex_out: ShaderAttribute,
    vertex_properties: Vec<(ShaderAttribute, ShaderAttribute)>,
    camera_matrix: ShaderAttribute,
}
impl SimpleVertexShader {
    pub fn default() -> Self {
        Self {
            vertex_in: ShaderAttribute::float4(0, String::from("position")),
            vertex_out: ShaderAttribute::output_float4(String::from("gl_Position")),
            vertex_properties: Vec::new(),
            camera_matrix: ShaderAttribute::uniform_mat4(String::from("camera_matrix")),
        }
    }

    pub fn from_vertex_map(properties: Vec<(ShaderAttribute, ShaderAttribute)>) -> Self {
        Self {
            vertex_in: ShaderAttribute::float4(0, String::from("position")),
            vertex_out: ShaderAttribute::output_float4(String::from("gl_Position")),
            vertex_properties: properties,
            camera_matrix: ShaderAttribute::uniform_mat4(String::from("camera_matrix")),
        }
    }
    pub fn get_vertex_in(&self) -> &ShaderAttribute {
        &self.vertex_in
    }
    pub fn get_vertex_out(&self) -> &ShaderAttribute {
        &self.vertex_out
    }
    pub fn get_vertex_properties(&self) -> &Vec<(ShaderAttribute, ShaderAttribute)> {
        &self.vertex_properties
    }
    pub fn get_camera_matrix(&self) -> &ShaderAttribute {
        &self.camera_matrix
    }
    pub fn apply_attributes(&self, gl: &glow::Context) {
        self.vertex_in.apply_attrib(gl);
        for property in &self.vertex_properties {
            property.0.apply_attrib(gl);
        }
    }
}

pub mod mesh_data {
    pub fn triangle() -> Vec<f32> {
        vec![
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0,
        ]
    }
    pub fn cube() -> Vec<f32> {
        vec![
            1.0, 1.0, -1.0, 1.0, //square 1
            1.0, -1.0, -1.0, 1.0,
            -1.0, -1.0, -1.0, 1.0,
            1.0, 1.0, -1.0, 1.0,
            -1.0, 1.0, -1.0, 1.0,
            -1.0, -1.0, -1.0, 1.0,

            1.0, 1.0, 1.0, 1.0, //square 2
            1.0, -1.0, 1.0, 1.0,
            -1.0, -1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            -1.0, 1.0, 1.0, 1.0,
            -1.0, -1.0, 1.0, 1.0,

            1.0, -1.0, 1.0, 1.0, //square 3
            1.0, -1.0, -1.0, 1.0,
            -1.0, -1.0, -1.0, 1.0,
            1.0, -1.0, 1.0, 1.0,
            -1.0, -1.0, 1.0, 1.0,
            -1.0, -1.0, -1.0, 1.0,

            1.0, 1.0, 1.0, 1.0, //square 4
            1.0, 1.0, -1.0, 1.0,
            -1.0, 1.0, -1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            -1.0, 1.0, 1.0, 1.0,
            -1.0, 1.0, -1.0, 1.0,

            -1.0, 1.0, 1.0, 1.0, //square 5
            -1.0, 1.0, -1.0, 1.0,
            -1.0, -1.0, -1.0, 1.0,
            -1.0, 1.0, 1.0, 1.0,
            -1.0, -1.0, 1.0, 1.0,
            -1.0, -1.0, -1.0, 1.0,

            1.0, 1.0, 1.0, 1.0, //square 6
            1.0, 1.0, -1.0, 1.0,
            1.0, -1.0, -1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, -1.0, 1.0, 1.0,
            1.0, -1.0, -1.0, 1.0,
        ]
    }

    pub fn cube_with_normals() -> Vec<f32> {
        vec![
            1.0, 1.0, -1.0, 1.0, 0.0, 0.0, -1.0,//square 1
            1.0, -1.0, -1.0, 1.0, 0.0, 0.0, -1.0,
            -1.0, -1.0, -1.0, 1.0, 0.0, 0.0, -1.0,
            1.0, 1.0, -1.0, 1.0, 0.0, 0.0, -1.0,
            -1.0, 1.0, -1.0, 1.0, 0.0, 0.0, -1.0,
            -1.0, -1.0, -1.0, 1.0, 0.0, 0.0, -1.0,

            1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, //square 2
            1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
            -1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
            -1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
            -1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 1.0,

            1.0, -1.0, 1.0, 1.0, 0.0, -1.0, 0.0, //square 3
            1.0, -1.0, -1.0, 1.0, 0.0, -1.0, 0.0,
            -1.0, -1.0, -1.0, 1.0, 0.0, -1.0, 0.0,
            1.0, -1.0, 1.0, 1.0, 0.0, -1.0, 0.0,
            -1.0, -1.0, 1.0, 1.0, 0.0, -1.0, 0.0,
            -1.0, -1.0, -1.0, 1.0, 0.0, -1.0, 0.0,

            1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, //square 4
            1.0, 1.0, -1.0, 1.0, 0.0, 1.0, 0.0,
            -1.0, 1.0, -1.0, 1.0, 0.0, 1.0, 0.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
            -1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
            -1.0, 1.0, -1.0, 1.0, 0.0, 1.0, 0.0,

            -1.0, 1.0, 1.0, 1.0, -1.0, 0.0, 0.0, //square 5
            -1.0, 1.0, -1.0, 1.0, -1.0, 0.0, 0.0,
            -1.0, -1.0, -1.0, 1.0, -1.0, 0.0, 0.0,
            -1.0, 1.0, 1.0, 1.0, -1.0, 0.0, 0.0,
            -1.0, -1.0, 1.0, 1.0, -1.0, 0.0, 0.0,
            -1.0, -1.0, -1.0, 1.0, -1.0, 0.0, 0.0,

            1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, //square 6
            1.0, 1.0, -1.0, 1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, -1.0, 1.0, 1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, -1.0, 1.0, 1.0, 0.0, 0.0,
        ]
    }

}


pub struct Mesh {
   vertexes: Vec<f32>,
   vbo: NativeBuffer,
   vao: NativeVertexArray,
   shader: VertexShader,
}

impl Mesh {
    pub fn create(vertexes: Vec<f32>, gl: &glow::Context) -> Result<Self, ResourceError> {
        Self::create_with_shader(vertexes, gl, VertexShader::default_simple())
    }

    pub fn create_with_shader(vertexes: Vec<f32>, gl: &glow::Context, shader: VertexShader) -> Result<Self, ResourceError> {
        let (vao, vbo);
        unsafe {
            (vao, vbo) = match Mesh::create_vertex_buffer(vertexes.clone(), &shader, gl) {
                Ok(pair) => pair,
                Err(err) => return Err(err),
            };
        }
        Ok(Self {
            vertexes,
            vbo,
            vao,
            shader,
        })
    }

    fn get_vertex_array(&self) -> NativeVertexArray {
        self.vao
    }

    fn _get_vertex_buffer(&self) -> NativeBuffer {
        self.vbo
    }

    fn get_vertex_count(&self) -> i32 {
        (self.vertexes.len() / 3) as i32
    }

    pub unsafe fn get_raw(&self) -> &[u8] {
        std::slice::from_raw_parts(
            self.vertexes.as_ptr() as *const u8,
            self.vertexes.len() * std::mem::size_of::<f32>())
    }

    pub fn get_shader(&self) -> &VertexShader {
        &self.shader
    }

    pub fn render(&self, gl: &glow::Context, camera: &Camera, program: &NativeProgram) {
        unsafe {
            gl.bind_vertex_array(Some(self.get_vertex_array()));
            let location = gl.get_uniform_location(*program, "camera_matrix");
            gl.uniform_matrix_4_f32_slice(location.as_ref(), false, &camera.get_transformation_matrix().to_cols_array());
            gl.draw_arrays(glow::TRIANGLES, 0, self.get_vertex_count());
            gl.bind_vertex_array(None);
        }
    }

    unsafe fn create_vertex_buffer(
        vertexes: Vec<f32>,
        shader: &VertexShader,
        gl: &glow::Context
    ) -> Result<(NativeVertexArray, NativeBuffer), ResourceError> {
        let vao = match gl.create_vertex_array() {
            Ok(array) => array,
            Err(err) => return Err(ResourceError::InvalidBuffer(err)),
        };
        gl.bind_vertex_array(Some(vao));

        let vbo = match gl.create_buffer() {
            Ok(buffer) => buffer,
            Err(err) => return Err(ResourceError::InvalidBuffer(err)),
        };

        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

        let vertexes_u8: &[u8] = core::slice::from_raw_parts(
            vertexes.as_ptr() as *const u8,
            vertexes.len() * core::mem::size_of::<f32>(),
        );

        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            vertexes_u8,
            glow::STATIC_DRAW);

        shader.apply_attributes(gl);
        gl.enable_vertex_attrib_array(0);

        gl.bind_vertex_array(None);

        Ok((vao, vbo))
    }

    pub fn destroy(&self, gl: glow::Context) {
        unsafe {
            gl.delete_buffer(self.vbo);
            gl.delete_vertex_array(self.vao);
        }
    }
}

