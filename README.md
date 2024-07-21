# RAPIDIMG (WIP)
## Overview
**RAPIDIMG** is a simple command-line application to compress images. This tool allows you to reduce the file size of images while maintaining an acceptable level of quality. It supports various image formats and provides a user-friendly way to handle image compression through the command line.

## Features
- Compress images to reduce file size.
- Support for multiple image formats (JPEG, PNG, BMP, etc.).
- Easy-to-use command-line interface.

## Requirements
Rust (version 1.54.0 or later)

## Installation
First, ensure you have Rust installed. You can install Rust using rustup:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Clone the repository and navigate to the project directory:
```sh
git clone https://github.com/flthibaud/rapidimg.git
cd rapidimg
```
Build the project using Cargo:

```sh
cargo build --release
```
The compiled binary will be located in the target/release directory.

## Usage
To compress an image, run the following command:

```sh
./target/release/rapidimg -i <input_image_path>
```
### Examples
to compress a single image:
```sh
./target/release/rapidimg -i example.jpg
```
to specify an output directory, use the `-o` option:
```sh
./target/release/rapidimg -i example.jpg -o compressed
```
to convert images to WebP format, use the `--webp` option:
```sh
./target/release/rapidimg -i images -o compressed --webp
```

### Command Line Options
- `-i, --input <INPUT>`: Specify the input image file or directory.
- `-o, --output <OUTPUT>`: (Optional) Specify the output directory for the compressed images.
- `--webp`: (Optional) Convert images to WebP format.

## Roadmap
- Add support for more image formats.
- Add support for image resizing.
- Add progress indicator.
- Add support for more conversion formats.
- Metadata preservation.

## Development
To contribute to this project, follow these steps:

1. Fork the repository.
2. Create a new branch (git checkout -b feature-branch).
3. Make your changes and commit them (git commit -am 'Add new feature').
4. Push to the branch (git push origin feature-branch).
5. Create a new Pull Request.

## License
This project is licensed under the MIT License.

## Contact
For any questions or suggestions, please open an issue.