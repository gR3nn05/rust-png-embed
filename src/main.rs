use clap::Parser;
use stego_encryptor::cli::Args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    args.execute()?;
    Ok(())
}