#[derive(Debug)]
pub enum ResourceError {
    MeshDoesNotExist,
    MaterialDoesNotExist,
    ObjectDoesNotExist,
    ObjectAlreadyExists,
    LoadingVertexSource(std::io::Error),
    LoadingFragmentSource(std::io::Error),
    InvalidShaderSource(String),
    InvalidBuffer(String),
}
