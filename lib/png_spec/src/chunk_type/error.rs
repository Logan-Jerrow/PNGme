use std::{error, fmt};

#[derive(Debug)]
pub enum Byte {
    Ancillary(u8),
    Private(u8),
    Reserved(u8),
    SafeToCopy(u8),
}

impl Byte {
    fn value(&self) -> u8 {
        match *self {
            Byte::Ancillary(b) => b,
            Byte::Private(b) => b,
            Byte::Reserved(b) => b,
            Byte::SafeToCopy(b) => b,
        }
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Byte::Ancillary(_) => write!(f, "ancillary"),
            Byte::Private(_) => write!(f, "private"),
            Byte::Reserved(_) => write!(f, "reserved"),
            Byte::SafeToCopy(_) => write!(f, "safe to copy"),
        }
    }
}

#[derive(Debug)]
pub enum ChunkTypeError {
    /// chunk types are resricted to A-Z and a-z
    InvalidByte(Byte),
    /// chunk types are 4 bytes
    InvalidLength(std::array::TryFromSliceError),
}

impl error::Error for ChunkTypeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ChunkTypeError::InvalidByte(_) => None,
            ChunkTypeError::InvalidLength(e) => Some(e),
        }
    }
}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkTypeError::InvalidByte(byte) => write!(
                f,
                "invalid '{byte}' byte: ['{}'/'{:#02X}'].\n\
                Chunk type codes are restricted to consist of uppercase or lowercase \
                ASCII letters (A-Z or a-z).",
                char::try_from(byte.value()).unwrap_or(char::REPLACEMENT_CHARACTER),
                byte.value()
            ),
            ChunkTypeError::InvalidLength(e) => {
                write!(f, "invalid length: chunk type codes are 4-bytes: {e}")
            }
        }
    }
}
