// Declare the simple_RGB folder as a module
mod simple_RGB;
mod matrix_conversion_RGB;
mod trenary;

use matrix_conversion_RGB::matrix_conversion;
// Import from the simple_RGB module
use simple_RGB::simple_RGB_embedding::complete_simpleRGB;
use simple_RGB::decrypt_simple_RGB::decrypt_simple_rgb;
use simple_RGB::rgb_dictionary::get_rgb_dictionary;
// Import from the trenary module
use trenary::rgb_embed_trenary::complete_trenary;
use trenary::rgb_decrypt_trenary::decrypt_trenary_rgb;
use std::fs;
use std::io;


fn read_file_to_string(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}

// main() function is the entry point of the program.
fn main() {

    let alphabet_used = get_rgb_dictionary();

    let message = read_file_to_string(r"C:\Users\HOME\RustSteg\Rust_Steg\text_files\expText.txt")
        .expect("Failed to read text file");

    // Convert image to matrix
    let image_matrix = matrix_conversion(r"C:\Users\HOME\RustSteg\Rust_Steg\images\BenPNG.png");

    // Process and save the image
    complete_simpleRGB(image_matrix, &message, r"C:\Users\HOME\RustSteg\Rust_Steg\images\Ben_exp.png")
        .expect("Failed to process and save the image");

    println!("Image processing completed successfully!");

    println!("Decrypted message: {}", decrypt_simple_rgb(&alphabet_used, r"C:\Users\HOME\RustSteg\Rust_Steg\images\BenPNG.png", r"C:\Users\HOME\RustSteg\Rust_Steg\images\Ben_exp.png"))


}