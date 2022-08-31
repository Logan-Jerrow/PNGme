use crate::chunk_type::ChunkType;
use crc::Crc;
use std::str::Utf8Error;

mod display;
pub mod error;
mod try_from;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Chunk {
    chunk_type: ChunkType,

    /// The data bytes appropriate to the chunk type, if any. This field can be of zero length.
    data: Vec<u8>,
}

impl Chunk {
    #[must_use]
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk { chunk_type, data }
    }

    /// A 4-byte unsigned integer giving the number of bytes in the chunk's data field. The length
    /// counts **only the data field**, not itself, the chunk type code, or the CRC. Zero is a valid
    /// length. Although encoders and decoders should treat the length as unsigned, its value must
    /// not exceed 2^31 bytes.
    pub fn data_length(&self) -> usize {
        self.data.len()
    }

    pub fn size(&self) -> usize {
        4 // data length
        + 4 // chunk type
        + self.data_length()
        + 4 // crc
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// A 4-byte CRC (Cyclic Redundancy Check) calculated on the preceding bytes in the chunk,
    /// including the chunk type code and chunk data fields, but **not** including the length
    /// field. The CRC is always present, even for chunks containing no data.
    pub fn crc(&self) -> u32 {
        let length = 4 + self.data_length(); // 4 byte chunk type + data length
        let mut bytes: Vec<u8> = Vec::with_capacity(length);
        bytes.extend(self.chunk_type.bytes());
        bytes.extend(self.data.iter());

        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        crc.checksum(&bytes)
    }

    pub fn data_as_string(&self) -> Result<String, Utf8Error> {
        std::str::from_utf8(&self.data).map(String::from)
    }

    pub fn data_as_string_lossy(&self) -> Result<String, Utf8Error> {
        std::str::from_utf8(&self.data).map(String::from)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        [
            u32::to_be_bytes(
                self.data_length()
                    .try_into()
                    .expect("Length invalid: should be 4-byte unsigned integer."),
            )
            .as_slice(),
            self.chunk_type.bytes().as_slice(),
            self.data.as_slice(),
            self.crc().to_be_bytes().as_slice(),
        ]
        .concat()
    }
}
