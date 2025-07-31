# rust-png-embed

A Rust CLI tool for embedding and extracting custom data within PNG image files. Designed for efficiency, cross-platform compatibility, and educational purposes.

## Features

- **Embed Data:** Seamlessly insert custom payloads (such as text or binary data) into PNG files without affecting image quality.
- **Extract Data:** Retrieve previously embedded data from PNG images with ease.
- **CLI Utility:** User-friendly command-line interface for embedding and extracting operations.
- **Safe and Performant:** Built using Rust's safety and concurrency guarantees.
- **Cross-Platform:** Works on Linux, macOS, and Windows.

## Installation

Clone the repository and build manually:

```sh
git clone https://github.com/gR3nn05/rust-png-embed.git
cd rust-png-embed
cargo build --release
```

## Usage


**Embed Data:**

```sh
rust-png-embed embed --input image.png --data secret.txt --output embedded.png
```

**Extract Data:**

```sh
rust-png-embed extract --input embedded.png --output extracted.txt
```


## How It Works

### PNG File Structure

A PNG file consists of a signature followed by a series of "chunks." Each chunk has:
- **Length** (4 bytes)
- **Chunk Type** (4 bytes, e.g., `IHDR`, `IDAT`, `IEND`)
- **Chunk Data** (variable length)
- **CRC** (4 bytes, for integrity)

The PNG standard supports **ancillary chunks**—custom, non-essential data blocks that do not affect image display or validity.

### Data Embedding Process

1. **Parse the PNG:** The tool reads and parses all chunks, preserving their order.
2. **Create a Custom Chunk:** Your payload is wrapped in a new ancillary chunk. Custom chunk names are chosen to avoid conflicts.
3. **Insert Custom Chunk:** The new chunk is inserted just before the final `IEND` chunk, which is the standard place for metadata or custom extensions.
4. **Write the New PNG:** All chunks, including the new one, are written to the output image. The image content is untouched, so it appears identical in any viewer.

### Data Extraction Process

1. **Parse the PNG:** The tool scans through all chunks.
2. **Locate the Custom Chunk:** It searches for the specific custom chunk type used by the too;.
3. **Extract the Payload:** The data is read from the chunk and saved to the specified output.

### Why This Is Safe and Effective

- **Visual Integrity:** Image data (`IDAT` chunk) is never modified, so the PNG looks the same.
- **Standard Compliance:** Ancillary chunks are part of the PNG spec; viewers ignore unknown chunks but preserve them.
- **File Integrity:** The PNG’s CRC mechanism ensures corrupted chunks are detected and ignored.
- **Stealth:** No pixel manipulation; data is stored invisibly and non-destructively.

---
## Use Cases

- Steganography and digital watermarking
- Secure information sharing
- Educational demonstrations of file structure manipulation


