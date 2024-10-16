use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}

#[derive(Subcommand)]
pub enum Command {
    Decode(DecodeArgs),
    Info(InfoArgs),
    Peers(PeersArgs),
}

#[derive(Parser)]
pub struct DecodeArgs {
    pub value: String,
}

#[derive(Parser)]
pub struct InfoArgs {
    pub path: PathBuf,
}

#[derive(Parser)]
pub struct PeersArgs {
    pub path: PathBuf,
}
