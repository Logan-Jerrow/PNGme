use crate::chunk_type::ChunkTypeError;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum ChunkError {
    IoError(std::io::Error),
    ChuckType(ChunkTypeError),
    InvalidLength(u32),
    Length { expected: u32, actual: u32 },
    Crc { expected: u32, actual: u32 },
}

impl From<ChunkTypeError> for ChunkError {
    fn from(v: ChunkTypeError) -> Self {
        Self::ChuckType(v)
    }
}

impl From<io::Error> for ChunkError {
    fn from(v: io::Error) -> Self {
        Self::IoError(v)
    }
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkError::IoError(e) => {
                write!(f, "encountered an io error while reading bytes: {e}")
            }
            ChunkError::ChuckType(e) => e.fmt(f),
            ChunkError::InvalidLength(e) => write!(
                f,
                "invalid chunk data length '{e}': length must not \
                exceed 2^32 bytes."
            ),
            ChunkError::Length { expected, actual } => write!(
                f,
                "length mismatch: given '{expected}' != '{actual}' actual"
            ),
            ChunkError::Crc { expected, actual } => {
                write!(f, "crc mismatch: given '{expected}' != '{actual}' actual")
            }
        }
    }
}

impl error::Error for ChunkError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ChunkError::IoError(e) => Some(e),
            ChunkError::ChuckType(e) => Some(e),
            _ => None,
        }
    }
}
