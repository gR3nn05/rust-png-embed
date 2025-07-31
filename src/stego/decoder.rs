use crate::crypto::decrypt_data;
use anyhow::Result;
use std::path::Path;

pub fn decode(input: &Path, output: &Path, passkey: &str) -> Result<()> {
    // Load the image
    let img = image::open(input)?;
    let rgb_img = img.to_rgb8();
    
    // First, extract the length of the data (4 bytes = 32 bits)
    let mut len_bits = Vec::new();
    let mut bit_count = 0;
    
    'len_loop: for (_, _, pixel) in rgb_img.enumerate_pixels() {
        for channel in 0..3 {
            if bit_count >= 32 {
                break 'len_loop;
            }
            
            let bit = match channel {
                0 => pixel.0[0] & 1,
                1 => pixel.0[1] & 1,
                2 => pixel.0[2] & 1,
                _ => unreachable!(),
            };
            
            len_bits.push(bit);
            bit_count += 1;
        }
    }
    
    // Convert length bits to u32
    let mut data_len = 0u32;
    for (i, &bit) in len_bits.iter().enumerate() {
        data_len |= (bit as u32) << (31 - i);
    }
    
    if data_len == 0 || data_len > 1_000_000 { // Sanity check
        return Err(anyhow::anyhow!("Invalid data length detected"));
    }
    
    // Extract the encrypted data
    let total_bits_needed = 32 + (data_len * 8); // length + data
    let mut all_bits = Vec::new();
    let mut bit_count = 0;
    
    'data_loop: for (_, _, pixel) in rgb_img.enumerate_pixels() {
        for channel in 0..3 {
            if bit_count >= total_bits_needed {
                break 'data_loop;
            }
            
            let bit = match channel {
                0 => pixel.0[0] & 1,
                1 => pixel.0[1] & 1,
                2 => pixel.0[2] & 1,
                _ => unreachable!(),
            };
            
            all_bits.push(bit);
            bit_count += 1;
        }
    }
    
    // Skip the length bits and extract data bits
    let data_bits = &all_bits[32..];
    
    // Convert bits to bytes
    let mut encrypted_data = Vec::new();
    for chunk in data_bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << (7 - i);
        }
        encrypted_data.push(byte);
    }
    
    // Decrypt the data
    let decrypted_data = decrypt_data(&encrypted_data, passkey)?;
    
    // Save to file
    std::fs::write(output, decrypted_data)?;
    println!("Data successfully extracted to {}", output.display());
    
    Ok(())
}