use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use eframe::egui;
use eframe::epaint::TextureHandle;
use image;

use super::resource_error::ResourceError;

pub fn load_shader(file_name: &str) -> Result<String, std::io::Error> {
    let dir = match shader_dir(file_name) {
        Ok(dir) => dir,
        Err(err) => return Err(std::io::Error::new(ErrorKind::Other, format!("{}: ({})", err, file_name))),
    };
    load_file(dir)
}

pub fn load_system_texture(image_name: &str, gl: &egui::Context) -> Result<TextureHandle, ResourceError> {
    let image = match load_system_image(image_name) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };
    Ok(gl.load_texture(image_name, image, egui::TextureFilter::Linear))
}

pub fn load_system_image(image_name: &str) -> Result<egui::ColorImage, ResourceError> {
    let path = match system_image_dir(image_name) {
        Ok(value) => value,
        Err(err) => return Err(ResourceError::LoadingFile(err, String::from(image_name))),
    };
    let image = match image::io::Reader::open(&path) {
        Ok(value) => value,
        Err(err) => return Err(ResourceError::LoadingFile(err, path.into_os_string().into_string().unwrap())),
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
        None => return Err(Error::new(ErrorKind::Other, "Could not find home dir"))
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

fn load_file(dir: PathBuf) -> Result<String, Error> {
    let file = match File::open(&dir) {
        Ok(file) => file,
        Err(err) => {
            let dir = match dir.to_str() {
                Some(dir) => dir,
                None => "__UNKNOWN_DIR__",
            };
            return Err(std::io::Error::new(ErrorKind::Other, format!("{} ({})", err, dir)));
        },
    };
    let mut buf_read = BufReader::new(file);
    let mut result = String::new();
    match buf_read.read_to_string(&mut result) {
        Ok(_) => (),
        Err(err) => return Err(err),
    }
    Ok(result)
}
