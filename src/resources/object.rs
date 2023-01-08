use glow::*;

use crate::resources::container::{Container, ContainerRef};
use crate::resources::material::Material;
use crate::resources::mesh::Mesh;

use crate::structures::camera::Camera;
use crate::util::error::Error;

use super::shader_gen;

pub struct Object {
    name: String,
    program: Option<NativeProgram>,
    material: ContainerRef<Material>,
    mesh: ContainerRef<Mesh>,
    vertex_shader: Option<NativeShader>,
    fragment_shader: Option<NativeShader>,
}

impl Object {

    pub fn create(material: ContainerRef<Material>, mesh: ContainerRef<Mesh>, container: &Container, gl: &glow::Context, name: &str) -> Result<Self, Error> {
        let mut result = Self {
            name: String::from(name),
            program: None,
            material,
            mesh,
            vertex_shader: None,
            fragment_shader: None,
        };
        unsafe {
            if let Err(err) = result.load_program(container, gl) {
                todo!("Error loading object: {:?}", err);
            }
        }
        Ok(result)
    }

    pub fn render(&self, gl: &glow::Context, camera: &Camera) {
        unsafe {
            gl.use_program(self.program);
        }
        if let Some(program) = self.program {
            self.mesh.lock().unwrap().render(gl, camera, &program);
        }
    }

    unsafe fn load_program(&mut self, container: &Container, gl: &glow::Context) -> Result<(), Error> {
        let program = gl.create_program().expect("Cannot create program");

        let mesh = self.mesh.lock().unwrap();
        let material = self.material.lock().unwrap();

        let vertex_shader_source = shader_gen::gen_vertex_shader(&mesh);
        println!("the vertex shader is:\n{}", vertex_shader_source);

        let vertex_shader = match Self::get_shader(&vertex_shader_source, glow::VERTEX_SHADER, gl) {
            Ok(shader) => shader,
            Err(err) => return Err(err),
        };

        let fragment_shader_source = shader_gen::gen_material_shader(&material, &mesh);
        println!("the fragment shader is:\n{}", fragment_shader_source);

        let fragment_shader = match Self::get_shader(&fragment_shader_source, glow::FRAGMENT_SHADER, gl) {
            Ok(shader) => shader,
            Err(err) => return Err(err),
        };


        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);

        gl.link_program(program);

        if !gl.get_program_link_status(program) {
            return Err(Error::invalid_gl_program(gl.get_program_info_log(program)));
        }

        self.vertex_shader = Some(vertex_shader);
        self.fragment_shader = Some(fragment_shader);
        self.program = Some(program);

        Ok(())

    }

    unsafe fn get_shader(
        shader_source: &str,
        shader_type: u32,
        gl: &glow::Context)-> Result<NativeShader, Error>
    {
        let shader = gl.create_shader(shader_type)
            .expect("Cannot create shader");
        gl.shader_source(shader, shader_source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            let info = gl.get_shader_info_log(shader);
            return Err(Error::invalid_shader_source(info));
        }
        Ok(shader)
    }

    pub fn get_name(&self) -> &str { &self.name }


    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            if let Some(program) = self.program {
                gl.delete_program(program);
            }
            if let Some(vertex_shader) = self.vertex_shader {
                gl.delete_shader(vertex_shader);
            }
            if let Some(fragment_shader) = self.fragment_shader {
                gl.delete_shader(fragment_shader);
            }
        }
    }
}
