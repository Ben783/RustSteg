// Uses a FIXED dictionary (each character is represented with a unique combination).
// Iterates through the entire altered image, comparing each pixel to the corresponding ORIGINAL pixel.
// If a difference is found, it is reverse looked up in the dictionary to find the embedded character.
// The size of the dictionary generated is determined at runtime.

use crate::matrix_conversion_RGB::matrix_conversion;
use std::collections::HashMap;

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


fn find_character_by_value(alphabet: &HashMap<char, [i32; 3]>, value: [i32; 3]) -> Option<char> {
    for (&character, &combo) in alphabet {
        if combo == value {
            return Some(character);
        }
    }
    None
}

pub fn decrypt_trenary_rgb(alphabet: &HashMap<char, [i32; 3]>, original_image_path: &str, alt_image_path: &str) -> String {
    let original_image_matrix = matrix_conversion(original_image_path);
    let alt_image_matrix = matrix_conversion(alt_image_path);

    let mut decrypted_message = String::new();

    for (y, row) in original_image_matrix.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let alt_pixel = alt_image_matrix[y][x];

            let r_diff = alt_pixel[0] as i32 - pixel[0] as i32;
            let g_diff = alt_pixel[1] as i32 - pixel[1] as i32;
            let b_diff = alt_pixel[2] as i32 - pixel[2] as i32;

            let modification = [r_diff, g_diff, b_diff];

            if modification != [0, 0, 0]{
                if let Some(character) = find_character_by_value(alphabet, [r_diff, g_diff, b_diff]) {
                decrypted_message.push(character);
                }
            }
        }
    }

    decrypted_message
}


// for testing - isolated binary
// fn main() {
//     let org_img_path = r"C:\Users\HOME\RustSteg\Rust_Steg\images\TREE.png";
//     let alt_img_path = r"C:\Users\HOME\RustSteg\Rust_Steg\images\TREEnaryexp.png";
//     println!(
//         "{}",
//         decrypt_trenary_rgb(&generate_trenary_rgb_dict(1), org_img_path, alt_img_path)
//     );
// }
