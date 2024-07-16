// Importation des bibliothèques nécessaires
use clap::Parser;
use image::{io::Reader as ImageReader, DynamicImage};
use image::ImageFormat;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;

/// Simple program to compress images
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // Macro pour définir la structure des arguments en ligne de commande
struct Args {
    /// Input image file
    #[arg(short, long)] // Spécifie les options de l'argument: court (-i) et long (--input)
    input: String, // Champ pour stocker le nom du fichier d'entrée

    /// Output image file
    #[arg(short, long)]
    output: String,
}

// Fonction pour charger une image à partir d'un fichier
fn load_image(input: &str) -> Result<DynamicImage, Box<dyn Error>> {
    let img = ImageReader::open(input)?.decode()?;
    Ok(img)
}

// Fonction pour convertir une taille en octets en une chaîne lisible par l'homme
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
    let output_dir = &args.output;

    // Vérifiez que le chemin de sortie est un répertoire
    let output_path = Path::new(output_dir);
    if !output_path.is_dir() {
        eprintln!("Error: Output path is not a directory. Please provide a valid directory path.");
        return;
    }

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

            // Devine le format de l'image
            let format = ImageReader::open(input)
                .expect("Failed to open image file")
                .with_guessed_format()
                .expect("Failed to guess image format")
                .format()
                .expect("Failed to determine image format");

            let input_filename = Path::new(input)
                .file_stem()
                .expect("Failed to get input file name")
                .to_str()
                .expect("Failed to convert file name to string");
            let input_extension = Path::new(input)
                .extension()
                .expect("Failed to get input file extension")
                .to_str()
                .expect("Failed to convert file extension to string");

            let output_file_path = output_path.join(format!("{}_compressed.{}", input_filename, input_extension));

            match format {
                ImageFormat::Jpeg => compress_jpeg(&img, &output_file_path),
                // ImageFormat::Png => compress_png(&img, &output_file_path),
                _ => println!("Format non pris en charge pour la compression"),
            }
        }
        Err(e) => eprintln!("Error loading image: {}", e),
    }
}

fn compress_jpeg(img: &DynamicImage, output_path: &Path) {
    let output_file = File::create(output_path).expect("Failed to create output file");

    let quality = 75;

    let mut jpeg_encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(output_file, quality);
    jpeg_encoder
        .encode_image(img)
        .expect("Failed to encode image as JPEG");

    println!("Image compressée et sauvegardée avec succès en JPEG !");
}
