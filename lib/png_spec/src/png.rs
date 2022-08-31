use crate::{chunk::Chunk, chunk_type::ChunkType};
use std::str::FromStr;

pub use self::error::PngError;

mod error;
mod trait_impls;

#[cfg(test)]
mod tests;

pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    /// PNG files always contain the following 8 bytes
    ///
    /// ['PNG file signature'](http://www.libpng.org/pub/png/spec/1.2/PNG-Rationale.html#R.PNG-file-signature)
    const STANDARD_HEADER: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Png {
        Png { chunks }
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk)
    }

    fn remove(&mut self, chunk_type: &ChunkType) -> Result<Chunk, PngError> {
        let index = self
            .chunks
            .iter()
            .position(|c| c.chunk_type() == chunk_type)
            .ok_or(PngError::ChunckTypeNotFound)?;

        Ok(self.chunks.swap_remove(index))
    }

    pub fn remove_chunk(&mut self, chunk_type: &ChunkType) -> Result<Chunk, PngError> {
        self.remove(chunk_type)
    }

    pub fn remove_str(&mut self, chunk_type: &str) -> Result<Chunk, PngError> {
        let chunk_type: ChunkType = chunk_type.parse()?;
        self.remove(&chunk_type)
    }

    pub fn header(&self) -> &[u8; 8] {
        &Self::STANDARD_HEADER
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        if let Ok(chunk_type) = ChunkType::from_str(chunk_type) {
            self.chunks.iter().find(|c| c.chunk_type() == &chunk_type)
        } else {
            None
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let h = self.header().iter();
        let c: Vec<u8> = self.chunks.iter().flat_map(Chunk::as_bytes).collect::<_>();

        h.copied().chain(c).collect::<Vec<u8>>()
    }
}
