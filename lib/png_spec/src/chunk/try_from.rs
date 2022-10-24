use super::{error::ChunkError, Chunk};
use crate::{chunk_type::ChunkType, util};
use std::io::BufReader;

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);

        // reading data from value
        let length = read_length(&mut reader)?;
        let chunk_type = read_chunk_type(&mut reader)?;
        let data = read_data(&mut reader, length)?;
        let crc = read_crc(&mut reader)?;

        assert_eq!(data.len(), length);

        let chunk = Self::new(chunk_type, data);

        // validating chunk
        if chunk.crc() != crc {
            return Err(ChunkError::Crc {
                expected: chunk.crc(),
                actual: crc,
            });
        }

        Ok(chunk)
    }
}

fn read_length(reader: &mut impl std::io::Read) -> Result<usize, ChunkError> {
    let mut length_byte: [u8; 4] = Default::default();
    reader.read_exact(&mut length_byte)?;
    let length = u32::from_be_bytes(length_byte);

    // Length can't exced 2^31 bytes.
    // or use i32.is_negitive
    if util::get_bit(length_byte[0], util::MOST_SIG) {
        return Err(ChunkError::InvalidLength(length));
    }

    Ok(length
        .try_into()
        .expect("invalid architecture: only support 32-bit+ architectures"))
}

fn read_chunk_type(reader: &mut impl std::io::Read) -> Result<ChunkType, ChunkError> {
    let mut chunk_type: [u8; 4] = Default::default();
    reader.read_exact(&mut chunk_type)?;
    Ok(ChunkType::try_from(chunk_type)?)
}

fn read_data(reader: &mut impl std::io::Read, length: usize) -> Result<Vec<u8>, ChunkError> {
    let mut data: Vec<u8> = vec![0; length];
    reader.read_exact(&mut data)?;
    Ok(data)
}

fn read_crc(reader: &mut impl std::io::Read) -> Result<u32, ChunkError> {
    let mut crc: [u8; 4] = Default::default();
    reader.read_exact(&mut crc)?;
    Ok(u32::from_be_bytes(crc))
}
