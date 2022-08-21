use crate::chunk_type::{ChunkType, ChunkTypeError};
use crc::Crc;
use std::{
    error,
    fmt::{self, Display},
    io::{self, BufReader, Read},
    str::Utf8Error,
};

#[derive(Debug)]
pub enum ChunkError {
    IoError(std::io::Error),
    ChuckType(ChunkTypeError),
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
            ChunkError::IoError(e) => write!(f, "encountered an io error while reading bytes: {e}"),
            ChunkError::ChuckType(e) => e.fmt(f),
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

#[derive(Debug)]
pub struct Chunk {
    chunk_type: ChunkType,

    /// The data bytes appropriate to the chunk type, if any. This field can be of zero length.
    data: Vec<u8>,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk { chunk_type, data }
    }

    /// A 4-byte unsigned integer giving the number of bytes in the chunk's data field. The length
    /// counts **only the data field**, not itself, the chunk type code, or the CRC. Zero is a valid
    /// length. Although encoders and decoders should treat the length as unsigned, its value must
    /// not exceed 2^31 bytes.
    pub fn data_length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn size(&self) -> u32 {
        4 + 4 + self.data_length() + 4
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// A 4-byte CRC (Cyclic Redundancy Check) calculated on the preceding bytes in the chunk,
    /// including the chunk type code and chunk data fields, but **not** including the length field.
    /// The CRC is always present, even for chunks containing no data.
    pub fn crc(&self) -> u32 {
        let chunk: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .chain(self.data.iter())
            .copied()
            .collect();

        Crc::<u32>::checksum(&Crc::<u32>::new(&crc::CRC_32_ISO_HDLC), &chunk)
    }

    pub fn data_as_string(&self) -> Result<String, Utf8Error> {
        std::str::from_utf8(&self.data).map(String::from)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        [
            self.data_length().to_be_bytes().as_slice(),
            self.chunk_type.bytes().as_slice(),
            self.data.as_slice(),
            self.crc().to_be_bytes().as_slice(),
        ]
        .concat()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);

        let mut length: [u8; 4] = Default::default();
        reader.read_exact(&mut length)?;
        let length = u32::from_be_bytes(length);

        let mut chunk_type: [u8; 4] = Default::default();
        reader.read_exact(&mut chunk_type)?;
        let chunk_type = ChunkType::try_from(chunk_type)?;

        let mut data: Vec<u8> = vec![0; length as usize];
        reader.read_exact(&mut data)?;
        let data = data;

        let mut crc: [u8; 4] = Default::default();
        reader.read_exact(&mut crc)?;
        let crc = u32::from_be_bytes(crc);

        // TODO: length must not exceed 2^31 bytes
        let data_len = data.len() as u32;
        if data_len != length {
            return Err(ChunkError::Length {
                expected: length,
                actual: data_len,
            });
        }

        let chunk = Self::new(chunk_type, data);
        if chunk.crc() != crc {
            return Err(ChunkError::Crc {
                expected: chunk.crc(),
                actual: crc,
            });
        }

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.data_length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.data_length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.data_length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.data_length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
