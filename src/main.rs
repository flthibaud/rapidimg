mod compress;
mod convert;
mod args;
mod utils;

use clap::Parser;
use image::ImageFormat;
use std::fs;
use std::path::{Path, PathBuf};
use args::Args;

fn main() {
    let args = Args::parse();
    let input = &args.input;
    let output_dir = args.output.as_deref();
    let input_path = Path::new(input);

    if input_path.is_dir() {
        process_directory(input_path, output_dir, args.webp);
    } else {
        process_single_image(input_path, output_dir, args.width, args.height, args.webp);
    }
}

/// Process all images in the given directory
fn process_directory(input_path: &Path, output_dir: Option<&str>, webp: bool) {
    let parent_dir_name = input_path.file_name().expect("Failed to get parent directory name");
    let output_base_path = match output_dir {
        Some(dir) => Path::new(dir).join(parent_dir_name),
        None => input_path.to_path_buf(),
    };

    if !output_base_path.exists() {
        fs::create_dir_all(&output_base_path).expect("Failed to create output directory");
    }

    match fs::read_dir(input_path) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.expect("Failed to read directory entry");
                let path = entry.path();
                if path.is_file() {
                    process_image(&path, output_dir, webp);
                }
            }
        }
        Err(e) => eprintln!("Error reading input directory: {}", e),
    }
}

/// Process a single image, resizing if necessary
fn process_single_image(input_path: &Path, output_dir: Option<&str>, width: Option<u32>, height: Option<u32>, webp: bool) {
    if width.is_some() || height.is_some() {
        resize_and_save_image(input_path, output_dir, width, height);
    }
    process_image(input_path, output_dir, webp);
}

/// Resize the image and save it to the output directory
fn resize_and_save_image(input_path: &Path, output_dir: Option<&str>, width: Option<u32>, height: Option<u32>) {
    let img = utils::load_image(input_path.to_str().unwrap()).expect("Failed to load image");
    let resized_img = utils::resize_image(&img, width, height);
    let (input_filename, input_extension) = utils::get_filename_and_extension(input_path);
    let output_file_path = match output_dir {
        Some(dir) => Path::new(dir).join(format!("{}_resized.{}", input_filename, input_extension)),
        None => input_path.with_file_name(format!("{}_resized.{}", input_filename, input_extension)),
    };
    resized_img.save(output_file_path).expect("Failed to save resized image");
    println!("Image redimensionnée et sauvegardée avec succès !");
}

/// Process the image: compress or convert to WebP
fn process_image(input: &Path, output_dir: Option<&str>, webp: bool) {
    println!("Processing image: {:?}", input);
    let input_str = input.to_str().unwrap();
    match utils::load_image(input_str) {
        Ok(img) => {
            let format = utils::guess_image_format(input_str);
            let (input_filename, input_extension) = utils::get_filename_and_extension(input);
            let output_file_path = get_output_file_path(input, output_dir, input_filename, input_extension, webp);

            if webp {
                convert_to_webp(input_str, &output_file_path);
            } else {
                compress_image(&img, input_str, &output_file_path, format);
            }
        }
        Err(e) => eprintln!("Error loading image: {}", e),
    }
}

/// Get the output file path based on the input and options
fn get_output_file_path(input: &Path, output_dir: Option<&str>, input_filename: String, input_extension: String, webp: bool) -> PathBuf {
    match output_dir {
        Some(dir) => {
            if webp {
                Path::new(dir).join(format!("{}.webp", input_filename))
            } else {
                Path::new(dir).join(format!("{}_compressed.{}", input_filename, input_extension))
            }
        }
        None => {
            let input_parent = input.parent().expect("Failed to get input file parent directory");
            if webp {
                input_parent.join(format!("{}.webp", input_filename))
            } else {
                input_parent.join(format!("{}_compressed.{}", input_filename, input_extension))
            }
        }
    }
}

/// Convert the image to WebP format
fn convert_to_webp(input_str: &str, output_file_path: &Path) {
    match convert::image_to_webp(&input_str.to_string(), &output_file_path) {
        Ok(_) => println!("Image compressée et sauvegardée avec succès en WebP !"),
        Err(e) => eprintln!("Error converting image to WebP: {}", e),
    }
}

/// Compress the image based on its format
fn compress_image(img: &image::DynamicImage, input_str: &str, output_file_path: &Path, format: ImageFormat) {
    match format {
        ImageFormat::Jpeg => compress::compress_jpeg(img, output_file_path),
        ImageFormat::Png => match compress::compress_png(input_str, output_file_path) {
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