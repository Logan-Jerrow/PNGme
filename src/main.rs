#![allow(dead_code)]

use std::fmt::Write;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

mod error {
    use crate::chunk_type;

    #[derive(Debug)]
    pub enum PngError {
        CT(chunk_type::ChunkTypeError),
    }

    impl std::error::Error for PngError {}
    impl std::fmt::Display for PngError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
}

/// PNG files always contain the following 8 bytes
///
/// ['PNG file signature'](http://www.libpng.org/pub/png/spec/1.2/PNG-Rationale.html#R.PNG-file-signature)
const PNG_8: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

fn main() -> anyhow::Result<()> {
    let png = std::fs::read("blackDragon.png").unwrap();

    let mut byte = String::from(" ");
    let mut dec = String::new();

    for b in png[0..8].iter() {
        write!(&mut byte, "{b:02x?} ").unwrap();
        write!(&mut dec, "{b:02} ").unwrap();
    }
    let byte = byte.trim_end();
    let dec = dec.trim();

    println!("{PNG_8:02x?}\n{byte}\n{dec}");

    Ok(())
}
