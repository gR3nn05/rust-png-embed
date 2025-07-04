use crate::crypto::{encrypt_data, derive_key};
use image::{GenericImageView, ImageBuffer};

pub fn encode(
    input_path: &str,
    output_path: &str,
    passkey: &str,
    data: Option<&str>,
) -> anyhow::Result<()> {
    let data = data.unwrap_or("DEFAULT_SECRET");
    let encrypted = encrypt_data(data, passkey)?;
    let mut img = image::open(input_path)?.to_rgba8();

    // Embed `encrypted` in LSBs of `img`...
    img.save(output_path)?;
    Ok(())
}