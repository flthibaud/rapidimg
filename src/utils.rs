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

pub fn resize_image(img: &DynamicImage, width: Option<u32>, height: Option<u32>) -> DynamicImage {
  let orig_width = img.width();
  let orig_height = img.height();

  // Handle case where both width and height are None
  if width.is_none() && height.is_none() {
      return img.clone();
  }

  // Calculate new dimensions
  let new_width = match (width, height) {
      (Some(w), None) => w,
      (None, Some(h)) => h * orig_width / orig_height,
      (Some(w), Some(_)) => w,
      (None, None) => unreachable!(), // This case is already handled above
  };

  let new_height = match (width, height) {
      (Some(_), Some(h)) => h,
      (Some(w), None) => w * orig_height / orig_width,
      (None, Some(h)) => h,
      (None, None) => unreachable!(), // This case is already handled above
  };

  img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
}