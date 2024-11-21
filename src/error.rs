use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug)]
pub enum RepTreeError {
    IoError(std::io::Error),
    UnexpectedComportment(String)
}

impl fmt::Display for RepTreeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RepTreeError::IoError(e) => write!(f, "IO error: {}",e),
            RepTreeError::UnexpectedComportment(message) => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for RepTreeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RepTreeError::IoError(e) => Some(e),
            _ => None,
        }
    }
}