use std::fmt;

#[derive(Debug)]
pub enum TeraclioError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    TemplateError(tera::Error),
    InvalidInput(String),
}

impl fmt::Display for TeraclioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TeraclioError::IoError(err) => write!(f, "IO error: {err}"),
            TeraclioError::JsonError(err) => write!(f, "JSON parsing error: {err}"),
            TeraclioError::TemplateError(err) => write!(f, "Template error: {err}"),
            TeraclioError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
        }
    }
}

impl std::error::Error for TeraclioError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TeraclioError::IoError(err) => Some(err),
            TeraclioError::JsonError(err) => Some(err),
            TeraclioError::TemplateError(err) => Some(err),
            TeraclioError::InvalidInput(_) => None,
        }
    }
}

impl From<std::io::Error> for TeraclioError {
    fn from(error: std::io::Error) -> Self {
        TeraclioError::IoError(error)
    }
}

impl From<serde_json::Error> for TeraclioError {
    fn from(error: serde_json::Error) -> Self {
        TeraclioError::JsonError(error)
    }
}

impl From<tera::Error> for TeraclioError {
    fn from(error: tera::Error) -> Self {
        TeraclioError::TemplateError(error)
    }
}

pub type Result<T> = std::result::Result<T, TeraclioError>;
