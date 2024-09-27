use image::{io::Reader as ImageReader, DynamicImage};
use image::ImageFormat;
use std::path::Path;
use std::error::Error;

// Fonction pour charger une image à partir d'un fichier
pub fn load_image(input: &str) -> Result<DynamicImage, Box<dyn Error>> {
  let img = ImageReader::open(input)?.decode()?;
  Ok(img)
}

// Fonction pour convertir une taille en octets en une chaîne lisible par l'homme
pub fn human_readable_size(bytes: u64) -> String {
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

pub fn guess_image_format(input: &str) -> ImageFormat {
  ImageReader::open(input)
      .expect("Failed to open image file")
      .with_guessed_format()
      .expect("Failed to guess image format")
      .format()
      .expect("Failed to determine image format")
}

pub fn get_filename_and_extension(input: &Path) -> (String, String) {
  let input_filename = input.file_stem()
      .expect("Failed to get input file name")
      .to_str()
      .expect("Failed to convert file name to string")
      .to_string();
  let input_extension = input.extension()
      .expect("Failed to get input file extension")
      .to_str()
      .expect("Failed to convert file extension to string")
      .to_string();
  (input_filename, input_extension)
}