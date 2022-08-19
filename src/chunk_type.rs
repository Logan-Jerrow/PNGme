use std::{fmt::Display, str::FromStr};

use super::error::PngError;

#[derive(Debug)]
pub enum ChunkTypeError {
    /// chunk types are resricted to A-Z and a-z
    InvalidByte,
    /// chunk types are 4 bytes
    InvalidLength,
}

impl std::error::Error for ChunkTypeError {}
impl std::fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::InvalidByte => write!(
                f,
                "invalid byte: chunk type codes are restricted to consist of uppercase and \
                lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal)."
            ),
            ChunkTypeError::InvalidLength => {
                write!(f, "invalid length: chunk type codes are 4-bytes")
            }
        }
    }
}

/// 4-byte chunk type code. For convenience in description and in
/// examining PNG files, type codes are restricted to consist of
/// uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90
/// and 97-122 decimal)
#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.0
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && Self::is_valid_bytes(self.0)
    }

    /// Ancillary bit: bit 5 of first byte
    /// 0 (uppercase) = critical, 1 (lowercase) = ancillary
    fn is_critical(&self) -> bool {
        self.0[0].is_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        self.0[1].is_ascii_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.0[2].is_ascii_uppercase()
    }

    fn is_safe_to_copy(&self) -> bool {
        !self.0[3].is_ascii_uppercase()
    }

    fn is_valid_byte(byte: u8) -> bool {
        // restricted to uper and lower case ASCII letters.
        byte.is_ascii_lowercase() || byte.is_ascii_uppercase()
    }

    fn is_valid_bytes(bytes: [u8; 4]) -> bool {
        bytes.into_iter().all(Self::is_valid_byte)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if !Self::is_valid_bytes(value) {
            // TODO: give precice bit(s) that are invalid
            return Err(ChunkTypeError::InvalidByte);
        }

        Ok(ChunkType(value))
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b: [u8; 4] = s
            .as_bytes()
            .try_into()
            .map_err(|e| ChunkTypeError::InvalidLength)?;
        ChunkType::try_from(b)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
