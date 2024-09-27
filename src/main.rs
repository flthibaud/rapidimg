mod compress;
mod convert;
mod args;
mod utils;

// Importation des bibliothèques nécessaires
use clap::Parser;
use image::ImageFormat;
use std::fs;
use std::path::Path;
use args::Args;

fn main() {
    let args = Args::parse();

    let input = &args.input;
    let output_dir = args.output.as_deref();

    let input_path = Path::new(input);

    if input_path.is_dir() {
        let parent_dir_name = input_path.file_name().expect("Failed to get parent directory name");
        let output_base_path = match output_dir {
            Some(dir) => Path::new(dir).join(parent_dir_name),
            None => input_path.to_path_buf(),
        };

        if !output_base_path.exists() {
            fs::create_dir_all(&output_base_path).expect("Failed to create output directory");
        }

        match fs::read_dir(input) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry.expect("Failed to read directory entry");
                    let path = entry.path();
                    if path.is_file() {
                        process_image(&path, Some(&output_base_path), args.webp);
                    }
                }
            }
            Err(e) => eprintln!("Error reading input directory: {}", e),
        }
    } else {
        process_image(input_path, output_dir.map(Path::new), args.webp);
    }
}

fn process_image(input: &Path, output_dir: Option<&Path>, webp: bool) {
    println!("Processing image: {:?}", input);
    let input_str = input.to_str().unwrap();
    match utils::load_image(input_str) {
        Ok(img) => {
            let format = utils::guess_image_format(input_str);
            let (input_filename, input_extension) = utils::get_filename_and_extension(input);

            let output_file_path = match output_dir {
                Some(dir) => {
                    if webp {
                        dir.join(format!("{}.webp", input_filename))
                    } else {
                        dir.join(format!("{}_compressed.{}", input_filename, input_extension))
                    }
                },
                None => {
                    let input_parent = input.parent().expect("Failed to get input file parent directory");
                    if webp {
                        input_parent.join(format!("{}.webp", input_filename))
                    } else {
                        input_parent.join(format!("{}_compressed.{}", input_filename, input_extension))
                    }
                }
            };

            if webp {
                match convert::image_to_webp(&input_str.to_string(), &output_file_path) {
                    Ok(_) => println!("Image compressée et sauvegardée avec succès en WebP !"),
                    Err(e) => eprintln!("Error converting image to WebP: {}", e),
                }
            } else {
                match format {
                    ImageFormat::Jpeg => compress::compress_jpeg(&img, &output_file_path),
                    ImageFormat::Png => match compress::compress_png(input_str, &output_file_path) {
                        Ok(stats) => {
                            println!("Image compressée et sauvegardée avec succès en PNG !");
                            println!("Taille d'origine: {}", utils::human_readable_size(stats.input_size));
                            println!("Nouvelle taille: {}", utils::human_readable_size(stats.output_size));
                            println!("Taux de compression: {:.2}%", stats.compression_ratio * 100.0);
                        }
                        Err(e) => eprintln!("Error compressing image: {}", e),
                    },
                    _ => println!("Format non pris en charge pour la compression"),
                }
            }
        }
        Err(e) => eprintln!("Error loading image: {}", e),
    }
}
