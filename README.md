# kersteg: LSB Steganography Tool

This application offers high-performance Least Significant Bit (LSB) steganography on images, allowing you to encode one image within another (referred to as the secret and decoy images) and decode it later.

LSB steganography is a method of hiding one image within another by embedding the secret image's data within the least significant bits of the decoy image. This program can encode and decode images using LSB steganography, preserving the appearance of the decoy image while hiding the secret image.

## Requirements

- Rust programming language (Latest Stable Version)
- The image crate for image handling
- The rayon crate for parallel processing

## How to Use

### Encoding

To encode a secret image within a decoy image, provide the path to the secret image, the decoy image, and the desired output file path, including the file type (e.g., .png, .jpg). The images must be of the same size for LSB steganography.

`$ cargo run /path/to/secret.png /path/to/decoy.png /path/to/output.png`

### Decoding

To decode an image previously encoded using the LSB technique, simply provide the path to the encoded image and the program will generate a decoded file with the original secret image.

`$ cargo run /path/to/encoded.png`

## Error Handling

The program will try to detect such errors as:

- Incorrect number of arguments
- Images of different sizes
- Inability to open or save image files
- Incorrect file paths

## Contribute

Contributions are very welcome! 
