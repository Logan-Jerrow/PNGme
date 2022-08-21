use std::io::{Read, Write};
use std::{fs::File, io::BufReader};

use anyhow::{bail, Context};
use pngme::chunk::Chunk;
use pngme::png::Png;

use super::*;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> anyhow::Result<()> {
    // let output =  args.output_file.map(|out| File::create(out)
    let mut output: Option<File> = None;
    if let Some(out) = args.output_file {
        output = Some(
            File::create(&out).with_context(|| format!("cannot create file {}", &out.display()))?,
        );
    }

    let file = File::open(&args.path)
        .with_context(|| format!("cannot open file {}", &args.path.display()))?;
    let size: usize = file.metadata()?.len().try_into()?;
    let mut reader = BufReader::new(file);

    // let mut buffer: Vec<u8> = vec![0u8; size];
    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    reader.read_to_end(&mut buffer)?;

    let chunk = Chunk::new(args.chunk_type, args.message.as_bytes().to_vec());
    let mut png = Png::try_from(&*buffer)?;
    png.append_chunk(chunk);

    if let Some(mut output) = output {
        output.write_all(&png.as_bytes())?;
        return Ok(());
    } else {
        bail!("no output file")
    }

    todo!()
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> anyhow::Result<()> {
    todo!()
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> anyhow::Result<()> {
    todo!()
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> anyhow::Result<()> {
    todo!()
}
