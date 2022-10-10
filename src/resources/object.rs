use glow::*;

use crate::resources::container::Container;

use crate::resources::resource_error::ResourceError;

pub struct Object {
    program: NativeProgram,
    //material_id: usize,
    mesh_id: usize,
}

impl Object {

    pub fn create(material_id: usize, mesh_id: usize, container: &Container, gl: &glow::Context) -> Result<Self, ResourceError> {
        let program;
        
        unsafe {
            program = Self::create_program(material_id, container, gl);
        }

        Ok(Self {
            program,
            //material_id,
            mesh_id,
        })
    }

    unsafe fn create_program(material_id: usize, container: &Container, gl: &glow::Context) -> NativeProgram {
        let program = gl.create_program().expect("Cannot create program");

        container.get_material(material_id).attach_shaders(gl, program);

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        container.get_material(material_id).detach_shaders(gl, program);

        program
    }

    pub fn render(&self, gl: &glow::Context, container: &Container) {
        unsafe {
            gl.use_program(Some(self.program));
        }
        container.get_mesh(self.mesh_id).render(gl);
    }

    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
        }
    }
}
