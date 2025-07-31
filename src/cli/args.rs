use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use crate::stego::{encoder, decoder};

#[derive(Parser)]
#[command(name = "stego_encryptor")]
#[command(about = "A steganography tool for hiding data in PNG images")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long)]
        passkey: String,
        #[arg(short, long)]
        data: String,
    },
    Decode {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long)]
        passkey: String,
    },
}

impl Args {
    pub fn parse_args() -> Result<Self> {
        Ok(Args::parse())
    }

    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Commands::Encode { input, output, passkey, data } => {
                encoder::encode(input, output, passkey, data)?;
            }
            Commands::Decode { input, output, passkey } => {
                decoder::decode(input, output, passkey)?;
            }
        }
        Ok(())
    }
}