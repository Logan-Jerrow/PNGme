use std::{
    fmt::{self, Display},
    str::FromStr,
};

use super::{error::ChunkTypeError, ChunkType};

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.into_iter().all(Self::is_valid_byte) {
            Ok(ChunkType {
                ancillary: value[0],
                private: value[1],
                reserved: value[2],
                safe_to_copy: value[3],
            })
        } else {
            // TODO: give precice bit(s) that are invalid
            Err(ChunkTypeError::InvalidByte)
        }
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
