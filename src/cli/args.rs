use clap::Parser;
pub mod args; 

#[derive(Parser)]
pub enum Args {
    Encode {
        input: String,
        output: String,
        passkey: String,
        data: Option<String>,
    },
    Decode {
        input: String,
        output: String,
        passkey: String,
    },
}

impl Args {
    pub fn execute(&self) -> anyhow::Result<()> {
        match self {
            Args::Encode { input, output, passkey, data } => {
                stego_encryptor::stego::encoder::encode(input, output, passkey, data)?;
            },
            Args::Decode { input, output, passkey } => {
                stego_encryptor::stego::decoder::decode(input, output, passkey)?;
            },
        }
        Ok(())
    }
}