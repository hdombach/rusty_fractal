use eframe::egui;
use eframe::epaint::TextureHandle;
use glam::Vec3;

use crate::resources::mesh::Mesh;
use crate::resources::material::Material;
use crate::resources::object::Object;

use crate::structures::camera::Camera;
use crate::util::error::{Error, ErrorKind};
use crate::util::ref_dict::*;

use super::mesh::mesh_data::{self, cube_with_normals};
use super::parser::parse_mesh;
use super::resource_file::{load_system_texture, mesh_dir};

pub struct Container {
    meshes: RefDict<String, Mesh>,
    materials: RefDict<String, Material>,
    objects: RefDict<String, Object>,
    textures: RefDict<String, TextureHandle>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            meshes: RefDict::new(),
            materials: RefDict::new(),
            objects: RefDict::new(),
            textures: RefDict::new(),
        }
    }

    pub fn default(gl: &glow::Context) -> Result<Self, Error> {
        let mut result = Self::new();
        result.add_material(&String::from("default"), Material::create_solid_color(Vec3::new(0.5, 0.5, 1.0))).unwrap();
        let gargoyle_mesh = parse_mesh(mesh_dir("gargoyle.ply").unwrap(), gl)?;
        let monkey_mesh = parse_mesh(mesh_dir("monkey.ply").unwrap(), gl)?;
        let cube_mesh = Mesh::create_with_shader(cube_with_normals(), gl, super::mesh::VertexShader::default_simple_with_normal())?;
        result.add_mesh("gargoyle_mesh", gargoyle_mesh)?;
        result.add_mesh("cube_mesh", cube_mesh)?;
        result.add_mesh("monkey_mesh", monkey_mesh)?;
        result.create_object("gargoyle", "default", "gargoyle_mesh", gl).unwrap();
        result.create_object("cube", "default", "cube_mesh", gl).unwrap();
        result.create_object("monkey", "default", "monkey_mesh", gl).unwrap();
        return Ok(result);
    }

    pub fn system_default(gl: &egui::Context) -> Result<Self, Error> {
        let mut result = Self::new();
        result.load_system_texture("right_rectangle.png", gl)?;
        result.load_system_texture("left_rectangle.png", gl)?;
        Ok(result)
    }

    pub fn create_object(
        &mut self,
        object_name: &str,
        material_name: &str,
        mesh_name: &str,
        gl: &glow::Context) -> Result<(), Error>
    {
        let material_id = match self.materials.add_reference(&String::from(material_name)) {
            Ok(id) => id,
            Err(err) => match err.get_kind() {
                ErrorKind::ValueDoesNotExist => return Err(Error::material_does_not_exist()),
                    _ => panic!("Invalid error"),
            }
        };
        let mesh_id = match self.meshes.add_reference(&String::from(mesh_name)) {
            Ok(id) => id,
            Err(err) => match err.get_kind() {
                ErrorKind::ValueDoesNotExist => return Err(Error::mesh_does_not_exist()),
                _ => panic!("Invalid error"),
            }
        };

        let new_object = match Object::create(
            material_id,
            mesh_id,
            self,
            gl)
        {
            Ok(object) => object,
            Err(err) => return Err(err),
        };

        match self.objects.add_value(&String::from(object_name), new_object) {
            Ok(_) => return Ok(()),
            Err(err) => match err.get_kind() {
                ErrorKind::ValueAlreadyExists => return Err(Error::object_already_exists()),
                _ => panic!("Invalid error")
            },
        }
    }

    pub fn add_mesh(&mut self,
                    name: &str,
                    mesh: Mesh) -> Result<(), Error>
    {
        self.meshes.add_value(&String::from(name), mesh)
    }

    pub fn add_material(
        &mut self,
        name: &str,
        material: Material) -> Result<(), Error>
    {
        self.materials.add_value(&String::from(name), material)
    }

    pub fn add_texture(&mut self,
                     name: &str,
                     texture: TextureHandle) -> Result<(), Error> {
        self.textures.add_value(&String::from(name), texture)
    }

    pub fn load_system_texture(&mut self,
                               name: &str,
                               gl: &egui::Context) -> Result<(), Error> {
        let texture = load_system_texture(name, gl)?;
        self.textures.add_value(&String::from(name), texture)?;
        Ok(())
    }

    pub fn add_object_reference(
        &mut self, name: &str) -> Result<usize, Error>
    {
        match self.objects.add_reference(&String::from(name)) {
            Ok(id) => return Ok(id),
            Err(err) => match err.get_kind() {
                ErrorKind::ValueDoesNotExist => return Err(Error::object_does_not_exist()),
                _ => panic!("Invalid error")
            },
        }
    }

    pub fn add_texture_reference(&mut self, name: &str) -> Result<usize, Error> {
        match self.objects.add_reference(&String::from(name)) {
            Ok(id) => return Ok(id),
            Err(err) => match err.get_kind() {
                ErrorKind::ValueDoesNotExist => return Err(Error::texture_does_not_exist()),
                _ => panic!("Invalid error")
            }
        }
    }

    pub fn get_material(&self, id: usize) -> &Material {
        self.materials.get_value(id)
    }
    pub fn get_mesh(&self, id: usize) -> &Mesh {
        self.meshes.get_value(id)
    }
    pub fn get_object(&self, id: usize) -> &Object {
        self.objects.get_value(id)
    }
    pub fn get_texture(&self, id: usize) -> &TextureHandle {
        self.textures.get_value(id)
    }

    pub fn get_material_id(&self, name: &str) -> Option<usize> {
        self.materials.get_value_id(String::from(name))
    }
    pub fn get_mesh_id(&self, name: &str) -> Option<usize> {
        self.meshes.get_value_id(String::from(name))
    }
    pub fn get_object_id(&self, name: &str) -> Option<usize> {
        self.objects.get_value_id(String::from(name))
    }
    pub fn get_texture_id(&self, name: &str) -> Option<usize> {
        self.textures.get_value_id(String::from(name))
    }

    pub fn render_object(&self, id: usize, gl: &glow::Context, camera: &Camera) {
        self.get_object(id).render(gl, self, camera);
    }
}
