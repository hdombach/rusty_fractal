/* a signle error type that will be used throughout the project */

use std::fmt::{Display, write};

pub type OptionsType = u8;

//Acts as a bitset
pub mod options {
    use crate::util::error::OptionsType;

    pub static NONE: OptionsType = 0;
    pub static RESOURCE_ERROR: OptionsType = 1;
    pub static REF_DICT_ERROR: OptionsType = 2;
    pub static PARSER_ERROR: OptionsType = 4;
}

#[derive(Debug)]
pub struct ParserUnknownCharContent {
    expected_char: char,
    received_char: char,
    line_number: usize,
    column_number: usize,
}
impl Display for ParserUnknownCharContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected {} but received {} at ({},{})", self.expected_char, self.received_char, self.line_number, self.column_number)
    }
}

#[derive(Debug)]
pub struct ParserUnknownWordContent {
    expected_word: String,
    received_word: String,
    line_number: usize,
    column_number: usize,
}
impl Display for ParserUnknownWordContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected {} but received {} at ({},{})", self.expected_word, self.received_word, self.line_number, self.column_number)
    }
}

#[derive(Debug)]
pub struct ParserUnknownLineContent {
    expected_line: String,
    received_line: String,
    line_number: usize,
}
impl Display for ParserUnknownLineContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected {} but received {} at {}", self.expected_line, self.received_line, self.line_number)
    }
}

#[derive(Debug)]
pub struct ParserInvalidAsciiIntContent {
    received_word: String,
    line_number: usize,
    column_number: usize,
}
impl Display for ParserInvalidAsciiIntContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot parse int from {} at ({},{})", self.received_word, self.line_number, self.column_number)
    }
}

#[derive(Debug)]
pub struct ParserUnknownStrContent {
    expected: String,
    received: String,
    line_number: usize,
    column_number: usize,
}
impl ParserUnknownStrContent {
    pub fn create(expected_str: &str, received_str: &str, line_number: usize, column_number: usize) -> Self {
        Self {
            expected: String::from(expected_str),
            received: String::from(received_str),
            line_number,
            column_number,
        }
    }
}
impl Display for ParserUnknownStrContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected {} but received {} at ({},{})", self.expected, self.received, self.line_number, self.column_number)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    /* Unknown errors */
    /* prob happens when I'm to lazy to manually convert errors */
    Unknown(String),
    UnknownIO(std::io::Error),
    UnknownImage(image::ImageError),
    /* Resource Errors */
    MeshDoesNotExist,
    MaterialDoesNotExist,
    ObjectDoesNotExist,
    TextureDoesNotExist,
    ObjectAlreadyExists,
    LoadingFile(std::io::Error, String),
    LoadingImage(image::ImageError),
    InvalidShaderSource(String),
    InvalidBuffer(String),
    InvalidGLProgram(String),
    InvalidHomeDir,
    /* Ref dict errors */
    ValueAlreadyExists,
    ValueDoesNotExist,
    /* Parser errors */
    ParserUnknownChar(ParserUnknownCharContent),
    ParserUnknownWord(ParserUnknownWordContent),
    ParserUnknownLine(ParserUnknownLineContent),
    ParserUnknownStr(ParserUnknownStrContent),
    ParserEndOfFile(),
    ParserInvalidAsciiInt(ParserInvalidAsciiIntContent),
    PlyParser(String),
}
impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(error_name) => write!(f, "Unknown error: {}", error_name),
            Self::UnknownIO(error) => write!(f, "Unknown IO error ({})", error),
            Self::UnknownImage(error) => write!(f, "Unknown image error({})", error),

            Self::MeshDoesNotExist => write!(f, "Mesh does not exist"),
            Self::MaterialDoesNotExist => write!(f, "Material does not exist"),
            Self::ObjectDoesNotExist => write!(f, "Object does not exist"),
            Self::TextureDoesNotExist => write!(f, "Texture does not exist"),
            Self::ObjectAlreadyExists => write!(f, "Object already exists"),
            Self::LoadingFile(error, file_name) => write!(f, "Could not find file {} ({})", file_name, error),
            Self::LoadingImage(error) => write!(f, "Problem loading image {}", error),
            Self::InvalidShaderSource(error) => write!(f, "Problem compiling shader ({})", error),
            Self::InvalidBuffer(error) => write!(f, "Problem creating buffer ({})", error),
            Self::InvalidGLProgram(error) => write!(f, "Problem loading OpenGL program ({})", error),
            Self::InvalidHomeDir => write!(f, "Could not find home dir"),

            Self::ValueAlreadyExists => write!(f, "Value already exists"),
            Self::ValueDoesNotExist => write!(f, "Value does not exist"),

            Self::ParserUnknownChar(content) => write!(f, "Parser error: Unknown char ({})", content),
            Self::ParserUnknownWord(word) => write!(f, "Parser error: Unknown word ({})", word),
            Self::ParserUnknownLine(line) => write!(f, "Parser error: Unknown line ({})", line),
            Self::ParserUnknownStr(content) => write!(f, "Parser error: Unknown str ({})", content),
            Self::ParserEndOfFile() => write!(f, "Parser error: Reached end of line."),
            Self::ParserInvalidAsciiInt(word) => write!(f, "Parser error: {} is not an int", word),
            Self::PlyParser(message) => write!(f, "Problem loading ply parser: {}", message),
        }
    }
}
impl PartialEq for ErrorKind {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    options: OptionsType,
}
impl Error {
    pub fn get_kind(self) -> ErrorKind {
        self.kind
    }
    pub fn get_options(self) -> OptionsType {
        self.options
    }

    pub fn is_resource_error(self) -> bool {
        self.options & options::RESOURCE_ERROR > 0
    }
    pub fn is_ref_dict_error(self) -> bool {
        self.options & options::REF_DICT_ERROR > 0
    }

    pub fn unknown(error_name: String) -> Self {
        Self { kind: ErrorKind::Unknown(error_name), options: options::NONE }
    }
    pub fn unknown_io(io_error: std::io::Error) -> Self {
        Self { kind: ErrorKind::UnknownIO(io_error), options: options::NONE }
    }
    pub fn unknown_image(image_error: image::ImageError) -> Self {
        Self { kind: ErrorKind::UnknownImage(image_error), options: options::NONE }
    }

    pub fn mesh_does_not_exist() -> Self {
        Self { kind: ErrorKind::MeshDoesNotExist, options: options::RESOURCE_ERROR }
    }
    pub fn material_does_not_exist() -> Self {
        Self { kind: ErrorKind::MaterialDoesNotExist, options: options::RESOURCE_ERROR }
    }
    pub fn object_does_not_exist() -> Self {
        Self { kind: ErrorKind::ObjectDoesNotExist, options: options::RESOURCE_ERROR }
    }
    pub fn texture_does_not_exist() -> Self {
        Self { kind: ErrorKind::TextureDoesNotExist, options: options::RESOURCE_ERROR }
    }
    pub fn object_already_exists() -> Self {
        Self { kind: ErrorKind::ObjectAlreadyExists, options: options::RESOURCE_ERROR }
    }
    pub fn loading_file(std_error: std::io::Error, file_name: String) -> Self {
        Self { kind: ErrorKind::LoadingFile(std_error, file_name), options: options::RESOURCE_ERROR }
    }
    pub fn loading_image(image_error: image::ImageError) -> Self {
        Self { kind: ErrorKind::LoadingImage(image_error), options: options::RESOURCE_ERROR }
    }
    pub fn invalid_shader_source(shader_error: String) -> Self {
        Self { kind: ErrorKind::InvalidShaderSource(shader_error), options: options::RESOURCE_ERROR }
    }
    pub fn invalid_buffer(buffer_error: String) -> Self {
        Self { kind: ErrorKind::InvalidBuffer(buffer_error), options: options::RESOURCE_ERROR }
    }
    pub fn invalid_gl_program(program_error: String) -> Self {
        Self { kind: ErrorKind::InvalidGLProgram(program_error), options: options::RESOURCE_ERROR }
    }
    pub fn invalid_home_dir() -> Self {
        Self { kind: ErrorKind::InvalidHomeDir, options: options::RESOURCE_ERROR }
    }

    pub fn value_already_exists() -> Self {
        Self { kind: ErrorKind::ValueDoesNotExist, options: options::REF_DICT_ERROR }
    }
    pub fn value_does_not_exist() -> Self {
        Self { kind: ErrorKind::ValueDoesNotExist, options: options::REF_DICT_ERROR }
    }

    pub fn parser_unknown_char(expected_char: char, received_char: char, line_number: usize, column_number: usize) -> Self {
        Self { kind: ErrorKind::ParserUnknownChar(ParserUnknownCharContent { expected_char, received_char, line_number, column_number }), options: options::PARSER_ERROR }
    }
    pub fn parser_unknown_word(expected_word: String, received_word: String, line_number: usize, column_number: usize) -> Self {
        Self { kind: ErrorKind::ParserUnknownWord(ParserUnknownWordContent { expected_word, received_word, line_number, column_number}), options: options::PARSER_ERROR }
    }
    pub fn parser_unknown_line(expected_line: String, received_line: String, line_number: usize) -> Self {
        Self { kind: ErrorKind::ParserUnknownLine(ParserUnknownLineContent { expected_line, received_line, line_number }), options: options::PARSER_ERROR }
    }
    pub fn parser_unknown_str(expected_str: &str, received_str: &str, line_number: usize, column_number: usize) -> Self {
        Self { kind: ErrorKind::ParserUnknownStr(ParserUnknownStrContent::create(expected_str, received_str, line_number, column_number)), options: options::PARSER_ERROR }
    }
    pub fn parser_end_of_file() -> Self {
        panic!();
        Self { kind: ErrorKind::ParserEndOfFile(), options: options::PARSER_ERROR }
    }
    pub fn parser_invalid_ascii_int(received_word: String, line_number: usize, column_number: usize) -> Self {
        Self { kind: ErrorKind::ParserInvalidAsciiInt(ParserInvalidAsciiIntContent { received_word, line_number, column_number }), options: options::PARSER_ERROR }
    }
    pub fn ply_parser(message: String) -> Self {
        Self { kind: ErrorKind::PlyParser(message), options: options::PARSER_ERROR }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        return self.kind == other.kind;
    }
}

impl From<image::ImageError> for Error {
    fn from(error: image::ImageError) -> Self {
        Self::unknown_image(error)
    }
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::unknown_io(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO: print options maybe
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for Error {}
