use clap::Parser;
use image::{io::Reader as ImageReader, DynamicImage};
use std::error::Error;
use std::fs;

/// Simple program to compress images
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input image file
    #[arg(short, long)]
    input: String,
}

fn load_image(input: &str) -> Result<DynamicImage, Box<dyn Error>> {
    let img = ImageReader::open(input)?.decode()?;
    Ok(img)
}

fn human_readable_size(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;

    if bytes >= GIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.2} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.2} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

fn main() {
    let args = Args::parse();

    let input = &args.input;
    match load_image(input) {
        Ok(img) => {
            println!("Image dimensions: {}x{}", img.width(), img.height());
            match fs::metadata(input) {
                Ok(metadata) => {
                    let size_in_bytes = metadata.len();
                    let human_readable = human_readable_size(size_in_bytes);
                    println!("Image size: {}", human_readable);
                }
                Err(e) => eprintln!("Error reading file metadata: {}", e),
            }
        }
        Err(e) => eprintln!("Error loading image: {}", e),
    }
}