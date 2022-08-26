use crate::args::*;
use anyhow::{bail, Context};
use pngme::chunk::Chunk;
use pngme::png::Png;
use std::io::{stdout, BufWriter, Read, Write};
use std::path::Path;
use std::{fs::File, io::BufReader};

fn read_png(path: impl AsRef<Path>) -> anyhow::Result<Png> {
    let file = File::open(&path)?;
    let size: usize = file.metadata()?.len() as usize;

    let mut reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::with_capacity(size);

    reader.read_to_end(&mut buffer)?;
    Ok(Png::try_from(&*buffer)?)
}

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> anyhow::Result<()> {
    // If creating output file fails then return early
    let output: Option<File> = if let Some(out) = args.output_file {
        Some(File::create(&out).with_context(|| format!("cannot create file {}", &out.display()))?)
    } else {
        None
    };

    let mut png = read_png(args.path)?;
    let chunk = Chunk::new(args.chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);

    if let Some(mut output) = output {
        output.write_all(&png.as_bytes())?;
    } else {
        let mut writer = BufWriter::new(stdout().lock());
        writer.write_all(&png.as_bytes())?;
        writer.flush()?;
    }

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> anyhow::Result<()> {
    let png = read_png(&args.path)?;
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type.to_string()) {
        if let Ok(s) = chunk.data_as_string() {
            println!("{s}",);
        } else {
            bail!("message not valid UTF-8")
        }
        // TODO: use arg option to use lossy, utf8, byte, hex
        // else {
        //     let s = String::from_utf8_lossy(chunk.data());
        //     println!("{s}");
        // }
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> anyhow::Result<()> {
    let mut png = read_png(&args.path)?;
    png.remove_chunk(&args.chunk_type)?;

    File::create(&args.path)?.write_all(&png.as_bytes())?;

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> anyhow::Result<()> {
    let png = read_png(&args.path)?;
    println!("{png}");
    Ok(())
}
