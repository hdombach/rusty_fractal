use std::collections::{HashMap, hash_map};
use std::rc::Rc;
use std::sync::{Mutex, Arc};

use eframe::egui;
use eframe::epaint::TextureHandle;
use glam::Vec3;

use crate::resources::mesh::Mesh;
use crate::resources::material::Material;
use crate::resources::object::Object;

use crate::structures::camera::Camera;
use crate::util::error::{Error, ErrorKind};
use crate::util::ref_dict::*;

use super::mesh::mesh_data::cube_with_normals;
use super::parser::parse_mesh;
use super::resource_file::{load_system_texture, mesh_dir};
use super::texture::Texture;

pub type ContainerRef<T> = Arc<Mutex<T>>;

pub fn new_container_ref<T>(value: T) -> ContainerRef<T> { Arc::new(Mutex::new(value)) }

pub struct Container {
    meshes: HashMap<String, ContainerRef<Mesh>>,
    materials: HashMap<String, ContainerRef<Material>>,
    objects: HashMap<String, ContainerRef<Object>>,
    textures: HashMap<String, ContainerRef<Texture>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
            materials: HashMap::new(),
            objects: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn default(gl: &glow::Context) -> Result<Self, Error> {
        let mut result = Self::new();
        result.add_material(Material::create_solid_color(Vec3::new(0.5, 0.5, 1.0), "default_material")).unwrap();
        let gargoyle_mesh = parse_mesh(mesh_dir("gargoyle.ply").unwrap(), gl, "gargoyle_mesh")?;
        let monkey_mesh = parse_mesh(mesh_dir("monkey.ply").unwrap(), gl, "monkey_mesh")?;
        let cube_mesh = Mesh::create_with_shader(cube_with_normals(), gl, super::mesh::VertexShader::default_simple_with_normal(), "cube_mesh")?;
        result.add_mesh(gargoyle_mesh)?;
        result.add_mesh(cube_mesh)?;
        result.add_mesh(monkey_mesh)?;
        result.create_object("gargoyle", "default_material", "gargoyle_mesh", gl).unwrap();
        result.create_object("cube", "default_material", "cube_mesh", gl).unwrap();
        result.create_object("monkey", "default_material", "monkey_mesh", gl).unwrap();
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
        let material = match self.materials.get(&String::from(material_name)) {
            Some(material) => material,
            None => {
                return Err(Error::material_does_not_exist(material_name));
            },
        };
        let mesh = match self.meshes.get(&String::from(mesh_name)) {
            Some(mesh) => mesh,
            None => {
                return Err(Error::mesh_does_not_exist(mesh_name));
            },
        };
        if self.objects.contains_key(&String::from(object_name)) {
            return Err(Error::object_already_exists(object_name));
        }

        let new_object = match Object::create(material.clone(), mesh.clone(), self, gl, object_name) {
            Ok(object) => object,
            Err(err) => return Err(err),
        };

        self.objects.insert(String::from(object_name), new_container_ref(new_object));
        Ok(())
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> Result<(), Error> {
        if self.meshes.contains_key(&String::from(mesh.get_name())) {
            return Err(Error::mesh_already_exists(mesh.get_name()));
        } else {
            self.meshes.insert(String::from(mesh.get_name()), new_container_ref(mesh));
            return Ok(());
        }
    }

    pub fn add_material(&mut self, material: Material) -> Result<(), Error> {
        if self.materials.contains_key(&String::from(material.get_name())) {
            return Err(Error::material_already_exists(material.get_name()));
        } else {
            self.materials.insert(String::from(material.get_name()), new_container_ref(material));
            return Ok(());
        }
    }

    pub fn add_texture_handle(&mut self, name: &str, texture: TextureHandle) -> Result<(), Error> {
        if self.textures.contains_key(&String::from(name)) {
            return Err(Error::texture_does_not_exist(name));
        } else {
            self.textures.insert(String::from(name), new_container_ref(Texture::new(texture, name)));
            Ok(())
        }
    }

    pub fn load_system_texture(&mut self,
                               name: &str,
                               gl: &egui::Context) -> Result<(), Error> {
        let texture = load_system_texture(name, gl)?;
        self.add_texture_handle(name, texture)?;
        Ok(())
    }

    pub fn get_material(&self, name: &str) -> Option<ContainerRef<Material>> {
        if let Some(material) = self.materials.get(name) {
            return Some(material.clone());
        }
        return None;
    }
    pub fn get_mesh(&self, name: &str) -> Option<ContainerRef<Mesh>> {
        if let Some(mesh) = self.meshes.get(name) {
            return Some(mesh.clone());
        }
        return None;
    }
    pub fn get_object(&self, name: &str) -> Option<ContainerRef<Object>> {
        if let Some(object) = self.objects.get(name) {
            return Some(object.clone());
        }
        return None;
    }
    pub fn get_texture(&self, name: &str) -> Option<ContainerRef<Texture>> {
        if let Some(texture) = self.textures.get(name) {
            return Some(texture.clone());
        }
        return None;
    }

    pub fn get_object_names(&self) -> hash_map::Keys<String, ContainerRef<Object>> {
        self.objects.keys()
    }
}
