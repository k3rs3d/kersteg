use std::env;
use std::fs;
use std::sync::{Arc, Mutex};
use std::error::Error;
use rayon::prelude::*;
use image::io::Reader as ImageReader;
use image::{Pixel, Rgb, RgbImage};

// Constants for encoding and decoding
const MASK_ENCODING: u8 = 0b1111_1110;
const MASK_DECODING: u8 = 0b0000_0001;
const SHIFT: u8 = 7;

// Function to load an image from a file
fn load_image(path: &str) -> Result<RgbImage, Box<dyn Error>> {
    if !fs::metadata(path)?.is_file() {
        return Err(format!("Error: {} is not a file.", path).into());
    }

    let img = ImageReader::open(path)?.decode()?.to_rgb8();
    Ok(img)
}

fn check_compatibility(secret_img: &RgbImage, decoy_img: &RgbImage) -> Result<(), Box<dyn Error>> {
    if secret_img.dimensions() != decoy_img.dimensions() {
        return Err("Error: Images must be the same size for LSB steganography.".into());
    }

    Ok(())
}

// Function to get the file extension, or return a default if none is found
fn get_file_extension(file_path: &str) -> Result<&str, Box<dyn Error>> {
    match std::path::Path::new(file_path).extension().and_then(std::ffi::OsStr::to_str) {
        Some(extension) => Ok(extension),
        None => Ok("png")
    }
}


// Function to process a pixel for LSB steganography, either encoding or decoding
fn process_pixel(secret_pixel: Rgb<u8>, decoy_pixel: Rgb<u8>, encoding: bool) -> Rgb<u8> {
    let mask = if encoding { MASK_ENCODING } else { MASK_DECODING };
    let mut output_pixel = Rgb([0; 3]);

    for i in 0..3 {
        output_pixel[i] = if encoding {
            // Hide secret image inside the decoy image
            (decoy_pixel[i] & mask) | (secret_pixel[i] >> SHIFT)
        } else {
            // Extract the secret image from the encoded image
            (secret_pixel[i] & mask) << SHIFT
        }
    }

    output_pixel
}

// Function to perform LSB steganography
fn perform_lsb_steganography(secret_img: &RgbImage, decoy_img: &RgbImage) -> Result<RgbImage, Box<dyn Error>> {
    let (width, height) = secret_img.dimensions();
    let output_img = Arc::new(Mutex::new(vec![0u8; (width * height * 3) as usize]));

    let secret_pixels: Vec<_> = secret_img.pixels().cloned().collect();
    let decoy_pixels: Vec<_> = decoy_img.pixels().cloned().collect();

    secret_pixels.par_iter().enumerate().for_each(|(i, secret_pixel)| {
        let decoy_pixel = &decoy_pixels[i];
        let processed_pixel = process_pixel(*secret_pixel, *decoy_pixel, true);
        let mut output = output_img.lock().unwrap();
        output[i*3..i*3+3].copy_from_slice(&processed_pixel.channels());
    });

    let raw_output = Arc::try_unwrap(output_img).unwrap().into_inner()?;
    RgbImage::from_raw(width, height, raw_output).ok_or("Failed to create image from raw data.".into())
}

// Function to decode a steganographic image
fn decode_lsb_steganography(encoded_img: &RgbImage) -> Result<RgbImage, Box<dyn Error>> {
    let (width, height) = encoded_img.dimensions();
    let output_img = Arc::new(Mutex::new(vec![0u8; (width * height * 3) as usize]));

    let encoded_pixels: Vec<_> = encoded_img.pixels().cloned().collect();

    encoded_pixels.par_iter().enumerate().for_each(|(i, encoded_pixel)| {
        let processed_pixel = process_pixel(*encoded_pixel, Rgb([0; 3]), false);
        let mut output = output_img.lock().unwrap();
        output[i*3..i*3+3].copy_from_slice(&processed_pixel.channels());
    });

    let raw_output = Arc::try_unwrap(output_img).unwrap().into_inner()?;
    RgbImage::from_raw(width, height, raw_output).ok_or("Failed to create image from raw data.".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let encoded_img = load_image(&args[1])?;
            let decoded_img = decode_lsb_steganography(&encoded_img)?;
            let file_extension = get_file_extension(&args[1])?;
            decoded_img.save(format!("decoded_output.{}", file_extension))?;
            println!("Decoding completed successfully.");
        },
        4 => {
            let output_path = &args[3];

            let secret_img = load_image(&args[1])?;
            let decoy_img = load_image(&args[2])?;

            check_compatibility(&secret_img, &decoy_img)?;
            let steganographic_img = perform_lsb_steganography(&secret_img, &decoy_img)?;
            steganographic_img.save(output_path)?;

            println!("Encoding completed successfully.");
        },
        _ => {
            if args.len() < 2 {
                return Err("Too few arguments. Please provide either one encoded image (to be decoded), or three arguments for encoding (the secret image, the decoy image, and finally the output file path including the desired file type).".into());
            } else {
                return Err("Too many arguments. Please provide either one encoded image (to be decoded), or three arguments for encoding (the secret image, the decoy image, and finally the output file path including the desired file type).".into());
            }
        },
    }
    
    Ok(())
}
