use std::{error, fmt};

#[derive(Debug)]
pub enum ChunkTypeError {
    /// chunk types are resricted to A-Z and a-z
    InvalidByte,
    /// chunk types are 4 bytes
    InvalidLength(std::array::TryFromSliceError),
}

impl error::Error for ChunkTypeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ChunkTypeError::InvalidByte => None,
            ChunkTypeError::InvalidLength(e) => Some(e),
        }
    }
}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkTypeError::InvalidByte => write!(
                f,
                "invalid byte: chunk type codes are restricted to consist of uppercase and \
                lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal)."
            ),
            ChunkTypeError::InvalidLength(e) => {
                write!(f, "invalid length: chunk type codes are 4-bytes: {e}")
            }
        }
    }
}
