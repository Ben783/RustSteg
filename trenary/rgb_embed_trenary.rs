use image::RgbImage;
use std::collections::HashMap;
use crate::matrix_conversion_RGB::matrix_conversion; // Rust_Steg is the project name, then the .rs file name in /src, then the function name. The file must be declared in lib.rs to be accessible.

fn calculate_greyscale_diff(org_r: u8, org_g: u8, org_b: u8, alt_r: u8, alt_g: u8, alt_b: u8) -> bool{
    fn calculate_greyscale(r: u8, g: u8, b: u8) -> u8 {
    ((0.299 * r as f32) + (0.587 * g as f32) + (0.114 * b as f32)).round() as u8
    }

    fn extract_lsb(value: u8) -> u8 {
        value & 1
    }

    let org_lsb = extract_lsb(calculate_greyscale(org_r, org_g, org_b));
    let alt_lsb = extract_lsb(calculate_greyscale(alt_r, alt_g, alt_b));

    if org_lsb == alt_lsb{
        true
    }
    else{
        false
    }
}

/// Generate a dictionary mapping chars to RGB-delta combos
fn generate_trenary_rgb_dict(n: i32) -> HashMap<char, [i32; 3]> {
    let mut dict = HashMap::new();

    let chars: Vec<char> = vec![
        'a','b','c','d','e','f','g','h','i',
        'j','k','l','m','n','o','p','q','r',
        's','t','u','v','w','x','y','z',' ',
        '.', '?'
    ];

    // Generate all combinations of [-n..=n]^3
    let mut combos = Vec::new();
    for x in -n..=n {
        for y in -n..=n {
            for z in -n..=n {
                combos.push([x, y, z]);
            }
        }
    }

    // Assign combos to characters until one runs out
    for (ch, combo) in chars.iter().zip(combos.iter()) {
        if combo != [0, 0, 0]{
            dict.insert(*ch, *combo);
        }
    }

    dict
}



fn check_if_rgb_in_bounds(r: i32, g: i32, b: i32) -> bool{
    if r >= 0 && r <= 255 && g >= 0 && g <= 255 && b >= 0 && b <= 255{
        true
    }
    else{
        false
    }
}

fn message_to_array(message: &str, dictionary: &HashMap<char, [i32; 3]>) -> Vec<[i32; 3]>{
    let mut message_array: Vec<[i32; 3]> = Vec::new();
    for character in message.chars(){
        match dictionary.get(&character){
            Some(value) => message_array.push(*value),
            None => {}
            
        }
    }
    message_array // [[1,0,0], [0,1,0], ...]
}

fn embed_trenary_rgb(mut image_matrix: Vec<Vec<[u8;3]>>, message: &str, dictionary: &HashMap<char, [i32; 3]>) -> Vec<Vec<[u8; 3]>>{
    let message_array = message_to_array(message, &dictionary);
    let mut message_index = 0;
    let message_length = message_array.len();

    for row in image_matrix.iter_mut(){
        for pixel in row.iter_mut(){
            if message_index < message_length{
                let [r, g, b] = *pixel;
                let new_r = r as i32 + message_array[message_index][0];
                let new_g = g as i32 + message_array[message_index][1];
                let new_b = b as i32 + message_array[message_index][2];
                if check_if_rgb_in_bounds(new_r, new_g, new_b){
                    if calculate_greyscale_diff(r, g, b, new_r as u8, new_g as u8, new_b as u8){
                        *pixel = [new_r as u8, new_g as u8, new_b as u8];
                        println!("Original {},{},{}", r, g, b);
                        println!("Alt {},{},{} \n", new_r, new_g, new_b);
                        message_index += 1;
                    }
                    else{
                        //println!("\n Pixel skipped. LSB mismatch");
                    }
                }
                else{
                    // skip this pixel.
                    //println!("\n Pixel skipped. Out of bounds - over/underflow. \n");
                }
            }
        }
    }

    image_matrix

}


fn reconstruct_image(matrix: Vec<Vec<[u8; 3]>>, output_path: &str) -> Result<(), image::ImageError>{
    let height = matrix.len();
    let width = matrix[0].len();
    
    // Flatten the matrix into a raw byte vector
    let mut raw_pixels: Vec<u8> = Vec::with_capacity(height * width * 3);
    
    for row in matrix {
        for pixel in row {
            raw_pixels.extend_from_slice(&pixel);
        }
    }
    
    // Create image from raw bytes
    let img = RgbImage::from_raw(width as u32, height as u32, raw_pixels)
        .expect("Failed to create image from raw data");
    
    img.save(output_path)

}

pub fn complete_trenary(image_matrix: Vec<Vec<[u8; 3]>>, message: &str, output_path: &str) -> Result<(), image:: ImageError>{
    let dictionary = generate_trenary_rgb_dict(1);
    let modified_matrix = embed_trenary_rgb(image_matrix, message, &dictionary);
    reconstruct_image(modified_matrix, output_path)
}

// for testing - isolated binary
// fn main(){
//     let org_image_path = r"C:\Users\HOME\RustSteg\Rust_Steg\images\TREE.png";
//     let output_path = r"C:\Users\HOME\RustSteg\Rust_Steg\images\TREEnaryexp.png";

//     let message = "aaa bbb ccc ddd";

//     let org_img_matrix = matrix_conversion(org_image_path);
//     complete_trenary(org_img_matrix, message, output_path).expect("Failed to embed message and/or save image");
// }