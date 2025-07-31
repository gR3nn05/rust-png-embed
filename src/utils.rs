use anyhow::Result;
use std::path::Path;



pub fn validate_image_path(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!("Image file does not exist: {}", path.display()));
    }
    
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        if !matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "bmp" | "gif") {
            return Err(anyhow::anyhow!("Unsupported image format: {}", ext));
        }
    } else {
        return Err(anyhow::anyhow!("No file extension found"));
    }
    
    Ok(())
}

pub fn ensure_output_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}