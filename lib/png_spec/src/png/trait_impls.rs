use std::{
    fmt::Display,
    io::{BufReader, Read},
};

use crate::chunk::Chunk;

use super::{error::PngError, Png};

impl Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.chunks() {
            writeln!(f, "{c}")?;
        }
        Ok(())
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = PngError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);

        let mut header: [u8; 8] = Default::default();
        reader.read_exact(&mut header)?;
        let header = header;
        if header != Self::STANDARD_HEADER {
            return Err(PngError::Header);
        }

        // A decoder may further verify that the next eight bytes contain an IHDR chunk header with
        // the correct chunk length; this will catch bad transfers that drop or alter null (zero)
        // bytes.
        //
        // let mut ihdr: [u8; 8] = Default::default();
        // reader.read_exact(&mut ihdr)?;
        // let _ihdr = ihdr;

        let mut chunks: Vec<Chunk> = vec![];

        let mut v = &value[8..];
        loop {
            if v.is_empty() {
                break;
            }
            let c = Chunk::try_from(v)?;
            let size = c.size();
            chunks.push(c);
            v = &v[size..];
        }

        Ok(Self::from_chunks(chunks))
    }
}
