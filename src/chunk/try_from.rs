use super::{error::ChunkError, Chunk};
use crate::chunk_type::ChunkType;
use std::io::{BufReader, Read};

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
