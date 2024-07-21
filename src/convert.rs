use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use image::DynamicImage;
use image::io::Reader as ImageReader;
use webp::{Encoder, WebPMemory};

pub fn image_to_webp(file_path: &String, output_path: &Path) -> Option<String> {
  let image = ImageReader::open(file_path);
  let image: DynamicImage = match image {
      Ok(img) => img.with_guessed_format().unwrap().decode().unwrap(),
      Err(e) => {
          println!("Error: {}", e);
          return None;
      }
  };

  println!("Entrez la qualité de compression (0-100) [par défaut: 75]: ");
  let mut quality_input = String::new();
  io::stdin().read_line(&mut quality_input).expect("Failed to read line");

  let quality = if quality_input.trim().is_empty() {
      75.0 // Valeur par défaut
  } else {
      match quality_input.trim().parse::<f32>() {
          Ok(q) if q >= 0.0 && q <= 100.0 => q,
          _ => {
              eprintln!("Valeur incorrecte. Utilisation de la qualité par défaut: 75");
              75.0
          }
      }
  };

  let encoder: Encoder = Encoder::from_image(&image).unwrap();
  let encoded_webp: WebPMemory = encoder.encode(quality);

  let mut webp_image = File::create(&output_path).unwrap();
  match webp_image.write_all(encoded_webp.as_ref()) {
      Err(e) => {
          println!("Error: {}", e);
          return None;
      }
      Ok(_) => {
          println!("Image compressée et sauvegardée avec succès en WebP !");
          return Some(output_path.to_str().unwrap().to_string());
      }
  }
}
