mod compress;

// Importation des bibliothèques nécessaires
use clap::Parser;
use image::{io::Reader as ImageReader, DynamicImage};
use image::ImageFormat;
use std::error::Error;
use std::fs;
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

    let input_path = Path::new(input);
    if input_path.is_dir() {
        match fs::read_dir(input) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry.expect("Failed to read directory entry");
                    let path = entry.path();
                    if path.is_file() {
                        process_image(&path, output_path);
                    }
                }
            }
            Err(e) => eprintln!("Error reading input directory: {}", e),
        }
    } else {
        process_image(input_path, output_path);
    }
}

fn process_image(input: &Path, output_path: &Path) {
    println!("Processing image: {:?}", input);
    let input_str = input.to_str().unwrap();
    match load_image(input_str) {
        Ok(img) => {
            let format = guess_image_format(input_str);
            let (input_filename, input_extension) = get_filename_and_extension(input);

            let output_file_path = output_path.join(format!("{}_compressed.{}", input_filename, input_extension));
            match format {
                ImageFormat::Jpeg => compress::compress_jpeg(&img, &output_file_path),
                ImageFormat::Png => match compress::compress_png(input_str, &output_file_path) {
                    Ok(stats) => {
                        println!("Image compressée et sauvegardée avec succès en PNG !");
                        println!("Taille d'origine: {}", human_readable_size(stats.input_size));
                        println!("Nouvelle taille: {}", human_readable_size(stats.output_size));
                        println!("Taux de compression: {:.2}%", stats.compression_ratio * 100.0);
                    }
                    Err(e) => eprintln!("Error compressing image: {}", e),
                }
                _ => println!("Format non pris en charge pour la compression"),
            }
        }
        Err(e) => eprintln!("Error loading image: {}", e),
    }
}

fn guess_image_format(input: &str) -> ImageFormat {
    ImageReader::open(input)
        .expect("Failed to open image file")
        .with_guessed_format()
        .expect("Failed to guess image format")
        .format()
        .expect("Failed to determine image format")
}

fn get_filename_and_extension(input: &Path) -> (String, String) {
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
