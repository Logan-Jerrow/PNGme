use clap::{Parser, Subcommand};
use png_spec::chunk_type::ChunkType;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,

    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    #[clap(value_parser)]
    pub path: PathBuf,

    #[clap(value_parser)] //= ChunkType::from_str)]
    pub chunk_type: ChunkType,

    #[clap(value_parser)]
    pub message: String,

    #[clap(value_parser)]
    pub output_file: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    #[clap(value_parser)]
    pub path: PathBuf,

    #[clap(value_parser)] //= ChunkType::from_str)]
    pub chunk_type: ChunkType,
}

#[derive(Debug, Parser)]
pub struct RemoveArgs {
    #[clap(value_parser)]
    pub path: PathBuf,

    #[clap(value_parser)] //= ChunkType::from_str)]
    pub chunk_type: ChunkType,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    #[clap(value_parser)]
    pub path: PathBuf,
}
