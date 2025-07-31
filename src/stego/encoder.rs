use crate::crypto::encrypt_data;
use image::ImageBuffer;
use anyhow::Result;
use std::path::Path;

pub fn encode(input: &Path, output: &Path, passkey: &str, data: &str) -> Result<()> {

    //load image and the encrypted message
    let img = image::open(input)?;
    let encrypted_data = encrypt_data(data.as_bytes(), passkey)?;
    
    //convert to RGB if not already
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    
    //check is the image is large enough for the data
    let max_bytes = (width * height * 3) / 8;
    if encrypted_data.len() > max_bytes as usize {
        return Err(anyhow::anyhow!("Image too small for data"));
    }
    
    //create a new image buffer and copy the image into it
    let mut new_img = ImageBuffer::new(width, height);
    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        new_img.put_pixel(x, y, *pixel);
    }
    
    //embed the length of the data first (4 bytes)
    let data_len = encrypted_data.len() as u32;
    let len_bytes = data_len.to_be_bytes();
    
    let mut bit_index = 0;
    let mut data_to_embed = Vec::new();
    data_to_embed.extend_from_slice(&len_bytes);
    data_to_embed.extend_from_slice(&encrypted_data);
    
    //embed data into LSBs
    'outer: for (_x, _y, pixel) in new_img.enumerate_pixels_mut() {
        for channel in 0..3 {
            if bit_index >= data_to_embed.len() * 8 {
                break 'outer;
            }
            
            let byte_index = bit_index / 8;
            let bit_offset = bit_index % 8;
            let bit = (data_to_embed[byte_index] >> (7 - bit_offset)) & 1;
            
            match channel {
                0 => pixel.0[0] = (pixel.0[0] & 0xFE) | bit,
                1 => pixel.0[1] = (pixel.0[1] & 0xFE) | bit,
                2 => pixel.0[2] = (pixel.0[2] & 0xFE) | bit,
                _ => unreachable!(),
            }
            
            bit_index += 1;
        }
    }
    
    //save the image
    new_img.save(output)?;
    println!("Data successfully embedded in {}", output.display());
    
    Ok(())
}