use clap::Parser;

/// Simple program to compress images
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input image file or directory
    #[arg(short, long)]
    pub input: String,

    /// Output directory (optional)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Convert images to WebP format
    #[arg(long)]
    pub webp: bool,

    /// Resize images to the specified width and height
    #[arg(short, long)]
    pub width: Option<u32>,
    pub height: Option<u32>,
}