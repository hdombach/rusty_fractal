use glam::Vec3;

use super::{resource_file::load_shader, mesh::{Mesh, SimpleVertexShader, VertexShader}, material::{Material, SolidColorMaterial}};

pub fn gen_material_shader(material: &Material, mesh: &Mesh) -> String {
    match material {
        Material::SolidColor(solid_material) => get_solid_color_material_shader(solid_material),
    }
}

fn get_solid_color_material_shader(shader: &SolidColorMaterial) -> String {
    let outputs = shader.get_color_out().as_fragment_shader_out();
    let vertex_code = format!("{} = {};", shader.get_color_out().name(), vec3_as_glsl_vec4(shader.get_color(), 1.0));

    let properties = vec![
        (String::from("OUTPUTS"), outputs),
        (String::from("FRAGMENT_CODE"), vertex_code),
    ];
    let template_fragment_source = match load_shader("template_fragment.glsl") {
        Ok(source) => source,
        Err(_) => todo!("add error handling"),
    };
    replace_comments(String::from(template_fragment_source), properties)
}

pub fn gen_vertex_shader(mesh: &Mesh) -> String {
    match mesh.get_shader() {
        VertexShader::Simple(simple_shader) => get_simple_vertex_shader(&simple_shader),
    }
}

fn get_simple_vertex_shader(shader: &SimpleVertexShader) -> String {
    let mut inputs = shader.get_vertex_in().as_vertex_shader_in();
    for property in shader.get_vertex_properties() {
        inputs += property.0.as_vertex_shader_in().as_str();
    }

    let mut outputs = String::new();
    for property in shader.get_vertex_properties() {
        outputs += property.1.as_vertex_shader_out().as_str();
    }

    let mut vertex_code = format!("vec4 _transformed_position = {} * {};\n", shader.get_camera_matrix().name(), shader.get_vertex_in().name());
    for property in shader.get_vertex_properties() {
        vertex_code += format!("{} = {};\n", property.0.name(), property.1.name()).as_str();
    }
    vertex_code += &format!("{} = _transformed_position;\n", shader.get_vertex_out().name());
    let properties = vec![
        (String::from("INPUTS"), inputs),
        (String::from("OUTPUTS"), outputs),
        (String::from("VERTEX_CODE"), vertex_code),
    ];
    let template_vertex_source = match load_shader("template_vertex.glsl") {
        Ok(source) => source,
        Err(_) => todo!("add error handling"),
    };
    replace_comments(String::from(template_vertex_source), properties)
}

fn vec3_as_glsl_vec3(value: Vec3) -> String {
    format!("vec3({}, {}, {})", value.x, value.y, value.z)
}
fn vec3_as_glsl_vec4(value: Vec3, alpha: f32) -> String {
    format!("vec4({}, {}, {}, {})", value.x, value.y, value.z, alpha)
}

pub fn replace_comments(source: String, replacements: Vec<(String, String)>) -> String {
    let mut result = source.clone();
    for replacement in replacements {
        result = replace_comment(&result, replacement.0, replacement.1);
    };
    result
}

pub fn replace_comment(source: &String, comment_mark: String, content: String) -> String {
    let comment_mark = String::new().to_owned() + "/* " + comment_mark.as_str() + " */";
    source.replace(comment_mark.as_str(), content.as_str())
}