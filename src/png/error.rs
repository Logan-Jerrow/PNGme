use crate::{chunk::error::ChunkError, chunk_type::ChunkTypeError};
use std::io;

#[derive(Debug)]
pub enum PngError {
    Chunk(ChunkError),
    ChunkType(ChunkTypeError),
    Io(io::Error),
    Header,
    ChunckTypeNotFound,
}

impl From<io::Error> for PngError {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}

impl From<ChunkError> for PngError {
    fn from(v: ChunkError) -> Self {
        PngError::Chunk(v)
    }
}

impl From<ChunkTypeError> for PngError {
    fn from(v: ChunkTypeError) -> Self {
        Self::ChunkType(v)
    }
}

impl std::error::Error for PngError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PngError::Chunk(e) => Some(e),
            PngError::ChunkType(e) => Some(e),
            PngError::Io(e) => Some(e),
            PngError::Header => None,
            PngError::ChunckTypeNotFound => None,
        }
    }
}

impl std::fmt::Display for PngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PngError::ChunkType(e) => e.fmt(f),
            PngError::Chunk(e) => e.fmt(f),
            PngError::Io(e) => e.fmt(f),
            PngError::Header => writeln!(f, "header is not png standard"),
            PngError::ChunckTypeNotFound => todo!(),
        }
    }
}
