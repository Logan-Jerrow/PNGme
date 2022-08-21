#![allow(dead_code)]

use clap::Parser;
use pngme::chunk_type::ChunkType;
use std::path::PathBuf;

mod commands;

// #[derive(Debug, Parser)]
// struct Args {
//     #[clap(subcommand)]
//     encode: Option<Commands>,
// }

#[derive(Debug, Parser)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    #[clap(value_parser)]
    path: PathBuf,

    #[clap(value_parser)] //= ChunkType::from_str)]
    chunk_type: ChunkType,

    #[clap(value_parser)]
    message: String,

    #[clap(value_parser)]
    output_file: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    #[clap(value_parser)]
    path: PathBuf,

    #[clap(value_parser)] //= ChunkType::from_str)]
    chunk_type: ChunkType,
}

#[derive(Debug, Parser)]
pub struct RemoveArgs {
    #[clap(value_parser)]
    path: PathBuf,

    #[clap(value_parser)] //= ChunkType::from_str)]
    chunk_type: ChunkType,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    #[clap(value_parser)]
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Commands::parse();

    match args {
        Commands::Encode(args) => commands::encode(args)?,
        Commands::Decode(_) => todo!(),
        Commands::Remove(_) => todo!(),
        Commands::Print(_) => todo!(),
    }
    // let png = std::fs::read("blackDragon.png").unwrap();

    // let mut byte = String::from(" ");
    // let mut dec = String::new();

    // for b in png[0..8].iter() {
    //     write!(&mut byte, "{b:02x?} ").unwrap();
    //     write!(&mut dec, "{b:02} ").unwrap();
    // }
    // let byte = byte.trim_end();
    // let dec = dec.trim();

    // println!("{PNG_8:02x?}\n{byte}\n{dec}");

    Ok(())
}
