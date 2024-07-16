// Importation des bibliothèques nécessaires
use clap::Parser; // Bibliothèque pour analyser les arguments de ligne de commande
use image::{io::Reader as ImageReader, DynamicImage}; // Bibliothèque pour manipuler les images
use std::error::Error; // Trait pour la gestion des erreurs
use std::fs; // Module pour les opérations sur le système de fichiers

/// Simple program to compress images
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // Macro pour définir la structure des arguments en ligne de commande
struct Args {
    /// Input image file
    #[arg(short, long)] // Spécifie les options de l'argument: court (-i) et long (--input)
    input: String, // Champ pour stocker le nom du fichier d'entrée
}

// Fonction pour charger une image à partir d'un fichier
fn load_image(input: &str) -> Result<DynamicImage, Box<dyn Error>> {
    // Ouvre le fichier image et le décode
    let img = ImageReader::open(input)?.decode()?;
    Ok(img) // Renvoie l'image décodée si succès
}

// Fonction pour convertir une taille en octets en une chaîne lisible par l'homme
fn human_readable_size(bytes: u64) -> String {
    // Définition des constantes pour les conversions d'unités
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;

    // Choisit le format approprié en fonction de la taille
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

// Fonction principale
fn main() {
    // Analyse les arguments de la ligne de commande
    let args = Args::parse();

    // Récupère le chemin du fichier d'entrée à partir des arguments
    let input = &args.input;

    // Tente de charger l'image
    match load_image(input) {
        Ok(img) => {
            // Si l'image est chargée avec succès, affiche ses dimensions
            println!("Image dimensions: {}x{}", img.width(), img.height());

            // Tente de récupérer les métadonnées du fichier
            match fs::metadata(input) {
                Ok(metadata) => {
                    // Si les métadonnées sont récupérées, affiche la taille du fichier en format lisible
                    let size_in_bytes = metadata.len();
                    let human_readable = human_readable_size(size_in_bytes);
                    println!("Image size: {}", human_readable);
                }
                Err(e) => eprintln!("Error reading file metadata: {}", e), // Affiche une erreur si la récupération échoue
            }
        }
        Err(e) => eprintln!("Error loading image: {}", e), // Affiche une erreur si le chargement de l'image échoue
    }
}
