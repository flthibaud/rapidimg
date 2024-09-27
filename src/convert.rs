use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use image::io::Reader as ImageReader;
use webp::{Encoder, WebPMemory};

pub fn image_to_webp(file_path: &str, output_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let image = ImageReader::open(file_path)?
        .with_guessed_format()?
        .decode()?;

    println!("Entrez la qualité de compression (0-100) [par défaut: 75]: ");
    let mut quality_input = String::new();
    io::stdin().read_line(&mut quality_input)?;

    let quality = quality_input.trim().parse::<f32>().unwrap_or(75.0).clamp(0.0, 100.0);

    let encoder: Encoder = Encoder::from_image(&image)?;
    let encoded_webp: WebPMemory = encoder.encode(quality);

    let mut webp_image = File::create(output_path)?;
    webp_image.write_all(encoded_webp.as_ref())?;

    Ok(output_path.to_str().unwrap().to_string())
}