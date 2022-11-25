use image;

use crate::util::ref_dict;

#[derive(Debug)]
pub enum ResourceError {
    MeshDoesNotExist,
    MaterialDoesNotExist,
    ObjectDoesNotExist,
    TextureDoesNotExist,
    ObjectAlreadyExists,
    LoadingFile(std::io::Error, String),
    LoadingImage(image::ImageError),
    InvalidShaderSource(String),
    InvalidBuffer(String),
    RefDictError(ref_dict::RefDictError),
    ProgramError(String),
}

impl From<ref_dict::RefDictError> for ResourceError {
    fn from(error: ref_dict::RefDictError) -> Self {
        ResourceError::RefDictError(error)
    }
}

impl From<image::ImageError> for ResourceError {
    fn from(error: image::ImageError) -> Self {
        ResourceError::LoadingImage(error)
    }
}
