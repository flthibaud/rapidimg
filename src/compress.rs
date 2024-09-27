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
    // Valider le chemin d'entrée
    if !Path::new(input_path).exists() {
        return Err(format!("Input file does not exist: {}", input_path));
    }

    // Configurer les options de compression
    let mut options = Options::max_compression();
    options.filter = indexset![
        RowFilter::None,
        RowFilter::Sub,
        RowFilter::Up,
        RowFilter::Average,
        RowFilter::Paeth,
    ];
    options.optimize_alpha = true;
    options.strip = StripChunks::Safe;
    options.bit_depth_reduction = true;
    options.color_type_reduction = true;
    options.palette_reduction = true;
    options.grayscale_reduction = true;
    options.interlace = Some(oxipng::Interlacing::None);

    // Lire les métadonnées du fichier d'entrée
    let input_size = std::fs::metadata(input_path)
        .map_err(|e| format!("Failed to read input file metadata: {}", e))?
        .len();

    // Effectuer l'optimisation PNG
    let infile = InFile::Path(input_path.into());
    let outfile = OutFile::Path {
        path: Some(output_path.to_path_buf()),
        preserve_attrs: false,
    };

    optimize(&infile, &outfile, &options)
        .map_err(|e| format!("Failed to optimize PNG: {}", e))?;

    // Lire les métadonnées du fichier de sortie
    let output_size = std::fs::metadata(output_path)
        .map_err(|e| format!("Failed to read output file metadata: {}", e))?
        .len();

    // Calculer le ratio de compression
    let compression_ratio = 1.0 - (output_size as f64 / input_size as f64);

    // Retourner les statistiques de compression
    Ok(CompressionStats {
        input_size,
        output_size,
        compression_ratio,
    })
}
