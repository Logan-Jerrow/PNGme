use std::{
    fmt::{self, Display},
    str::FromStr,
};

use super::{
    error::{Byte, ChunkTypeError},
    ChunkType,
};

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let [ancillary, private, reserved, safe_to_copy] = value;

        if !Self::is_valid_byte(ancillary) {
            return Err(ChunkTypeError::InvalidByte(Byte::Ancillary(ancillary)));
        }

        if !Self::is_valid_byte(private) {
            return Err(ChunkTypeError::InvalidByte(Byte::Private(private)));
        }

        if !Self::is_valid_byte(reserved) {
            return Err(ChunkTypeError::InvalidByte(Byte::Reserved(reserved)));
        }

        if !Self::is_valid_byte(safe_to_copy) {
            return Err(ChunkTypeError::InvalidByte(Byte::SafeToCopy(safe_to_copy)));
        }

        Ok(ChunkType {
            ancillary,
            private,
            reserved,
            safe_to_copy,
        })
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b: [u8; 4] = s
            .as_bytes()
            .try_into()
            .map_err(ChunkTypeError::InvalidLength)?;
        ChunkType::try_from(b)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.bytes()).unwrap())
    }
}
