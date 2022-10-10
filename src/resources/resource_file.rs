use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::io::{Error, ErrorKind};

pub fn load_shader(file_name: &str) -> Result<String, std::io::Error> {
    let dir = match shader_dir(file_name) {
        Ok(dir) => dir,
        Err(err) => return Err(err),
    };
    load_file(dir)
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

fn load_file(dir: PathBuf) -> Result<String, Error> {
    let file = match File::open(dir) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut buf_read = BufReader::new(file);
    let mut result = String::new();
    match buf_read.read_to_string(&mut result) {
        Ok(_) => (),
        Err(err) => return Err(err),
    }
    Ok(result)
}
