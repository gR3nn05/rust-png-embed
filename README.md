# rust-png-embed

A Rust library and CLI tool for embedding and extracting custom data within PNG image files. Designed for efficiency, cross-platform compatibility, and educational purposes.

## Features

- **Embed Data:** Seamlessly insert custom payloads (such as text or binary data) into PNG files without affecting image quality.
- **Extract Data:** Retrieve previously embedded data from PNG images with ease.
- **CLI Utility:** User-friendly command-line interface for embedding and extracting operations.
- **Safe and Performant:** Built using Rust's safety and concurrency guarantees.
- **Cross-Platform:** Works on Linux, macOS, and Windows.

## Installation

You can install `rust-png-embed` using Cargo:

```sh
cargo install rust-png-embed
```

Alternatively, clone the repository and build manually:

```sh
git clone https://github.com/gR3nn05/rust-png-embed.git
cd rust-png-embed
cargo build --release
```

## Usage

### CLI Tool

**Embed Data:**

```sh
rust-png-embed embed --input image.png --data secret.txt --output embedded.png
```

**Extract Data:**

```sh
rust-png-embed extract --input embedded.png --output extracted.txt
```

### Library

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rust-png-embed = "0.1"
```

Example usage in Rust:

```rust
use rust_png_embed::{embed_data, extract_data};

fn main() {
    let input_image = "image.png";
    let output_image = "embedded.png";
    let data = b"Secret payload!";
    embed_data(input_image, data, output_image).expect("Embedding failed");

    let extracted = extract_data(output_image).expect("Extraction failed");
    assert_eq!(data, &extracted[..]);
}
```

## How It Works

This tool leverages the PNG file format's extensibility by inserting custom ancillary chunks to carry your data. These chunks do not interfere with image rendering and remain undetectable to standard image viewers.

## Use Cases

- Steganography and digital watermarking
- Secure information sharing
- Educational demonstrations of file structure manipulation


