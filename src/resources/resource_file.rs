use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use eframe::egui;
use eframe::egui::TextBuffer;
use eframe::epaint::TextureHandle;
use image;

use crate::util::error::Error;

pub fn load_shader(file_name: &str) -> Result<String, Error> {
    let dir = match shader_dir(file_name) {
        Ok(dir) => dir,
        Err(err) => return Err(err),
    };
    load_file(dir)
}

pub fn load_system_texture(image_name: &str, gl: &egui::Context) -> Result<TextureHandle, Error> {
    let image = match load_system_image(image_name) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };
    Ok(gl.load_texture(image_name, image, egui::TextureFilter::Linear))
}

pub fn load_system_image(image_name: &str) -> Result<egui::ColorImage, Error> {
    let path = match system_image_dir(image_name) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };
    let image = match image::io::Reader::open(&path) {
        Ok(value) => value,
        Err(err) => return Err(Error::loading_file(err, String::from(image_name))),
    };
  
    let image = image.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

fn asset_dir() -> Result<PathBuf, Error> {
    let mut result = match dirs::home_dir() {
        Some(dir) => dir,
        None => return Err(Error::invalid_home_dir()),
    };
    result.push(".rusty_fractal");
    Ok(result)
}

fn shader_dir(shader_name: &str) -> Result<PathBuf, Error> {
    let mut result = match asset_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(err),
    };
    result.push("shaders");
    result.push(shader_name);
    Ok(result)
}

fn system_image_dir(image_name: &str) -> Result<PathBuf, Error> {
    let mut result = match asset_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(err),
    };
    result.push("system_images");
    result.push(image_name);
    Ok(result)
}

pub fn mesh_dir(mesh_name: &str) -> Result<PathBuf, Error> {
    let mut result = match asset_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(err),
    };
    result.push("meshes");
    result.push(mesh_name);
    Ok(result)
}

pub fn load_file(dir: PathBuf) -> Result<String, Error> {
    let file = match File::open(&dir) {
        Ok(file) => file,
        Err(err) => {
            let dir = match dir.to_str() {
                Some(dir) => dir,
                None => "__UNKNOWN_DIR__",
            };
            return Err(Error::loading_file(err, String::from(dir.as_str())));
        },
    };
    let mut buf_read = BufReader::new(file);
    let mut result = String::new();
    match buf_read.read_to_string(&mut result) {
        Ok(_) => (),
        Err(err) => return Err(Error::loading_file(err, String::from(dir.to_str().unwrap()))),
    }
    Ok(result)
}

pub fn load_file_raw_vec(dir: PathBuf) -> Result<Vec<u8>, Error> {
    let mut file = match File::open(&dir) {
        Ok(file) => file,
        Err(err) => {
            let dir = match dir.to_str() {
                Some(dir) => dir,
                None => "__UNKNOWN_DIR__",
            };
            return Err(Error::loading_file(err, String::from(dir)));
        },
    };
    let mut result = Vec::new();
    file.read_to_end(&mut result)?;
    return Ok(result);
}
