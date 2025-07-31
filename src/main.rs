use anyhow::Result;
use stego_encryptor::cli::args::Args;

fn main() -> Result<()> {
    let args = Args::parse_args()?;
    args.execute()
}