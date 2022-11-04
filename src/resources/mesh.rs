use std::mem;

use glow::*;
use crate::{resources::resource_error::ResourceError, structures::camera::Camera};

pub struct Mesh {
   vertexes: Vec<f32>,
   vbo: NativeBuffer,
   vao: NativeVertexArray,
}

impl Mesh {
    pub fn create_default_triangle(
        gl: &glow::Context) -> Result<Self,ResourceError>
    {
        let vertexes: Vec<f32> = vec![
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0
        ];
        Mesh::create_from_vertexes(vertexes, gl)
    }

    pub fn create_cube(
        gl: &glow::Context) -> Result<Self, ResourceError>
    {
        let vertexes: Vec<f32> = vec![
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


        ];
        Mesh::create_from_vertexes(vertexes, gl)
    }

    pub fn create_from_vertexes(
        vertexes: Vec<f32>,
        gl: &glow::Context) -> Result<Self, ResourceError>
    {
        let (vao, vbo);
        unsafe {
            (vao, vbo) = match Mesh::create_vertex_buffer(vertexes.clone(), gl) {
                Ok(pair) => pair,
                Err(err) => return Err(err),
            };
        }
        Ok(Self {
            vertexes,
            vbo,
            vao
        })
    }

    fn get_vertex_array(&self) -> NativeVertexArray {
        self.vao
    }

    fn get_vertex_buffer(&self) -> NativeBuffer {
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

        gl.vertex_attrib_pointer_f32(0, 4, glow::FLOAT, false, (mem::size_of::<f32>() as i32) * 4, 0);
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

