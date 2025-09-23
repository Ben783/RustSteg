// image_matrix:  (Vec<Vec<[u8; 3]>>)
// [
// [[r,g,b],[r,g,b],[r,g,b]...],
// [[r,g,b],[r,g,b],[r,g,b]...], 
//  ...]

use crate::matrix_conversion_RGB::matrix_conversion;
use std::collections::HashMap;
use crate::simple_RGB::rgb_dictionary::get_rgb_dictionary;

fn split_image_matrix(image_matrix: Vec<Vec<[u8; 3]>>, pixel_dist: u8) -> Vec<Vec<[u8; 3]>> {
    let step = pixel_dist as usize; // pixel stepping distance
    let mut strips = Vec::new();

    // iterate over rows first (row-major order)
    for row in &image_matrix {
        // iterate over columns in steps of 'step' to get slices of the row
        for col_start in (0..row.len()).step_by(step) {
            // extract a slice of the row starting from col_start to col_start + step
            let slice = row[col_start..(col_start + step).min(row.len())].to_vec();
            // only slices of full length 'step' are processed
            if slice.len() == step {
                strips.push(slice); 
            }
        }
    }
    //println!("{:?}", strips);
    strips // returns [ 
    // [[r,g,b], [r,g,b]], 
    // [[r,g,b], [r,g,b]],
    //  ... ] 
}


fn find_character_by_value(alphabet: &HashMap<char, (i32, i32)>, value: i32) -> Option<char> {
    for (&character, &(a, _b)) in alphabet {
        if a == value {
            return Some(character);
        }
    }
    None
}

pub fn decrypt_dist_rgb(original_image_path: &str, alt_image_path: &str, pixel_dist: u8, dictionary: &HashMap<char, (i32, i32)>) -> String{
    let org_img_matrix = matrix_conversion(original_image_path);
    let alt_img_matrix = matrix_conversion(alt_image_path);

    let org_strips = split_image_matrix(org_img_matrix, pixel_dist);
    let alt_strips = split_image_matrix(alt_img_matrix, pixel_dist);

    let mut decrypted_message = String::new();


    for (strip_index, strip) in org_strips.iter().enumerate(){
        let mut strip_diff = 0;
        let mut pixel_diff = 0;
        for (pixel_index, pixel) in strip.iter().enumerate(){
            let alt_pixel = alt_strips[strip_index][pixel_index];

            let r_diff = alt_pixel[0] as i32 - pixel[0] as i32;
            let g_diff = alt_pixel[1] as i32 - pixel[1] as i32;
            let b_diff = alt_pixel[2] as i32 - pixel[2] as i32;

            let pixel_diff = r_diff.abs() + g_diff.abs() + b_diff.abs();
            strip_diff += pixel_diff;
        }

        if let Some(character) = find_character_by_value(dictionary, strip_diff.abs()) {
            decrypted_message.push(character);
        }

        //if strip_diff != 0{
            //println!("{}", strip_diff.abs());
            //println!("org pixels: {}", strip_index);
        //}

    }
    decrypted_message
}


// for running the binary in /bin...
// fn main(){
//     let org_image_path = r"C:\Users\HOME\RustSteg\Rust_Steg\images\TREE.png";
//     let alt_path = r"C:\Users\HOME\RustSteg\Rust_Steg\images\dist_TREE.png";
//     let pixel_dist = 3;

//     let message = decrypt_dist_rgb(org_image_path, alt_path, pixel_dist, &get_rgb_dictionary());
//     println!("{}", message);
// }