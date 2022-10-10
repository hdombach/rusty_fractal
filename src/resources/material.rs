use glow::*;
use crate::resources::resource_error::ResourceError;
use crate::resources::resource_file::*;

pub struct Material {
    vertex_shader_source: String, //idk if the sources need to be saved
    fragment_shader_source: String,
    vertex_shader: NativeShader,
    fragment_shader: NativeShader,
}

impl Material {
    pub fn create_default(
        gl: &glow::Context)-> Result<Self, ResourceError>
    {
        Self::create_from_file(
            DEFAULT_VERTEX_SHADER_SOURCE,
            DEFAULT_FRAGMENT_SHADER_SOURCE,
            gl)
    }

    pub fn create_from_file(
        vertex_shader_file: &str,
        fragment_shader_file: &str,
        gl: &glow::Context) -> Result<Self, ResourceError>
    {

        let vertex_shader_source = match load_shader(vertex_shader_file) {
            Ok(source) => source,
            Err(err) => return Err(ResourceError::LoadingVertexSource(err)),
        };

        let fragment_shader_source = match load_shader(fragment_shader_file) {
            Ok(source) => source,
            Err(err) => return Err(ResourceError::LoadingFragmentSource(err)),
        };

        Self::create(&vertex_shader_source, &fragment_shader_source, gl)
    }

    pub fn create(
        vertex_shader_source: &str,
        fragment_shader_source: &str,
        gl: &glow::Context) -> Result<Self, ResourceError>
    {
        unsafe {
            let vertex_shader = match Self::get_shader(
                vertex_shader_source,
                glow::VERTEX_SHADER, gl)
            {
                Ok(shader) => shader,
                Err(err) => return Err(err),
            };

            let fragment_shader = match Self::get_shader(
                fragment_shader_source,
                glow::FRAGMENT_SHADER, gl)
            {
                Ok(shader) => shader,
                Err(err) => return Err(err),
            };

            Ok( Self {
                vertex_shader_source: String::from(vertex_shader_source),
                fragment_shader_source: String::from(fragment_shader_source),
                vertex_shader,
                fragment_shader,
            })
        }
    }

    pub unsafe fn attach_shaders(
        &self,
        gl: &glow::Context,
        program: NativeProgram)
    {
        gl.attach_shader(program, self.vertex_shader);
        gl.attach_shader(program, self.fragment_shader);
    }

    pub unsafe fn detach_shaders(
        &self,
        gl: &glow::Context,
        program: NativeProgram)
    {
        gl.detach_shader(program, self.vertex_shader);
        gl.detach_shader(program, self.fragment_shader);
    }

    unsafe fn get_shader(
        shader_source: &str,
        shader_type: u32,
        gl: &glow::Context)-> Result<NativeShader, ResourceError>
    {
        let shader = gl.create_shader(shader_type)
            .expect("Cannot create shader");
        gl.shader_source(shader, shader_source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            let info = gl.get_shader_info_log(shader);
            return Err(ResourceError::InvalidShaderSource(info));
        }
        Ok(shader)
    }

    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_shader(self.vertex_shader);
            gl.delete_shader(self.fragment_shader);
        }
    }
}

const DEFAULT_VERTEX_SHADER_SOURCE: &str = "default_vertex.glsl";
const DEFAULT_FRAGMENT_SHADER_SOURCE: &str = "default_fragment.glsl";

