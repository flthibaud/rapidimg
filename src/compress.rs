use std::path::Path;
use oxipng::{indexset, optimize, InFile, Options, OutFile, RowFilter, StripChunks};
use image::DynamicImage;
use std::fs::File;
use std::io;

pub struct CompressionStats {
  pub input_size: u64,
  pub output_size: u64,
  pub compression_ratio: f64,
}

pub fn compress_jpeg(img: &DynamicImage, output_path: &Path) {
  let output_file = File::create(output_path).expect("Failed to create output file");

  println!("Entrez la qualité de compression (0-100) [par défaut: 75]: ");
  let mut quality_input = String::new();
  io::stdin().read_line(&mut quality_input).expect("Failed to read line");
  
  let quality = if quality_input.trim().is_empty() {
      75 // Valeur par défaut
  } else {
      match quality_input.trim().parse::<u8>() {
          Ok(q) if q <= 100 => q,
          _ => {
              eprintln!("Valeur incorrecte. Utilisation de la qualité par défaut: 75");
              75
          }
      }
  };

  let mut jpeg_encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(output_file, quality);
  jpeg_encoder
      .encode_image(img)
      .expect("Failed to encode image as JPEG");

  println!("Image compressée et sauvegardée avec succès en JPEG !");
}

pub fn compress_png(input_path: &str, output_path: &Path) -> Result<CompressionStats, String> {
  let mut options = Options::max_compression();
  options.filter = indexset![
      RowFilter::None,
      RowFilter::Sub,
      RowFilter::Up,
      RowFilter::Average,
      RowFilter::Paeth,
  ]; // Utilise tous les types de filtres de ligne
  options.optimize_alpha = true; // Optimise les canaux alpha
  options.strip = StripChunks::Safe; // Supprime les métadonnées inutiles
  options.bit_depth_reduction = true; // Réduit la profondeur de bits si possible
  options.color_type_reduction = true; // Réduit le type de couleur si possible
  options.palette_reduction = true; // Réduit la palette de couleurs si possible
  options.grayscale_reduction = true; // Convertit en niveaux de gris si possible
  options.interlace = Some(oxipng::Interlacing::None); // Désactive l'entrelacement pour réduire la taille

  let infile = InFile::Path(input_path.into());
  let outfile = OutFile::Path {
      path: Some(output_path.to_path_buf()),
      preserve_attrs: false,
  };

  let input_size = std::fs::metadata(input_path)
      .map_err(|e| format!("Failed to read input file metadata: {}", e))?
      .len();

  optimize(&infile, &outfile, &options)
    .map_err(|e| format!("Failed to optimize PNG: {}", e))?;

  let output_size = std::fs::metadata(output_path)
      .map_err(|e| format!("Failed to read output file metadata: {}", e))?
      .len();

  let compression_ratio = 1.0 - (output_size as f64 / input_size as f64);

  Ok(CompressionStats {
      input_size,
      output_size,
      compression_ratio,
  })
}
