use crate::resources::mesh::Mesh;
use crate::resources::material::Material;
use crate::resources::object::Object;

use crate::util::ref_dict::*;

use crate::resources::resource_error::ResourceError;

pub struct Container {
    meshes: RefDict<String, Mesh>,
    materials: RefDict<String, Material>,
    objects: RefDict<String, Object>,
}

impl Container {
    pub fn default(gl: &glow::Context) -> Result<Self, ResourceError> {
        let mut result = Self {
            meshes: RefDict::new(),
            materials:  RefDict::new(),
            objects: RefDict::new(),
        };
        match Material::create_default(gl) {
            Ok(material) => result.add_material(&String::from("default"), material).unwrap(),
            Err(err) => return Err(err),
        };
        match Mesh::create_default_triangle(gl) {
            Ok(mesh) => result.add_mesh(&String::from("default"), mesh).unwrap(),
            Err(err) => return Err(err),
        };
        result.create_object(
            &String::from("default"),
            &String::from("default"),
            &String::from("default"),
            gl).unwrap();
        return Ok(result);
    }

    pub fn create_object(
        &mut self,
        object_name: &String,
        material_name: &String,
        mesh_name: &String,
        gl: &glow::Context) -> Result<(), ResourceError>
    {
        let material_id = match self.materials.add_reference(&material_name) {
            Ok(id) => id,
            Err(err) => match err {
                RefDictError::ValueDoesNotExist =>return Err(
                    ResourceError::MaterialDoesNotExist),
                _ => panic!("Invalid error"),
            },
        };
        let mesh_id = match self.meshes.add_reference(&mesh_name) {
            Ok(id) => id,
            Err(err) => match err {
                RefDictError::ValueDoesNotExist => return Err(
                    ResourceError::MeshDoesNotExist),
                _ => panic!("Invalid error"),
            },
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

        match self.objects.add_value(object_name, new_object) {
            Ok(_) => return Ok(()),
            Err(err) => match err {
                RefDictError::ValueAlreadyExists => return Err(
                    ResourceError::ObjectAlreadyExists),
                _ => panic!("Invalid error")
            },
        }
    }

    pub fn add_mesh(&mut self,
                    name: &String,
                    mesh: Mesh) -> Result<(), RefDictError>
    {
        self.meshes.add_value(name, mesh)
    }

    pub fn add_material(
        &mut self,
        name: &String,
        material: Material) -> Result<(), RefDictError>
    {
        self.materials.add_value(name, material)
    }

    pub fn add_object_reference(
        &mut self, name: String) -> Result<usize, ResourceError>
    {
        match self.objects.add_reference(&name) {
            Ok(id) => return Ok(id),
            Err(err) => match err {
                RefDictError::ValueDoesNotExist => return Err(
                    ResourceError::ObjectDoesNotExist),
                _ => panic!("Invalid error")
            },
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

    pub fn get_material_id(&self, name: String) -> Option<usize> {
        self.materials.get_value_id(name)
    }
    pub fn get_mesh_id(&self, name: String) -> Option<usize> {
        self.meshes.get_value_id(name)
    }
    pub fn get_object_id(&self, name: String) -> Option<usize> {
        self.objects.get_value_id(name)
    }

    pub fn render_object(&self, id: usize, gl: &glow::Context) {
        self.get_object(id).render(gl, self);
    }
}
