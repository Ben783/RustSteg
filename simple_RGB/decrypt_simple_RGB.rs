// image_matrix (mxn):
//    [
//    [[r,g,b],[r,g,b],[r,g,b], ...],
//    [[r,g,b],[r,g,b],[r,g,b], ...],
//    ...
//    ]

use crate::matrix_conversion_RGB::matrix_conversion;

use std::collections::HashMap;

pub fn decrypt_simple_rgb(alphabet: &HashMap<char, (i32, i32)>, original_image_path: &str, alt_image_path: &str) -> String{
    let original_image_matrix = matrix_conversion(original_image_path);
    let alt_image_matrix = matrix_conversion(alt_image_path);

    let mut decrypted_message = String::new();

    for (y, row) in original_image_matrix.iter().enumerate(){
        for (x, pixel) in row.iter().enumerate(){
            let alt_pixel = alt_image_matrix[y][x];

            let r_diff = (alt_pixel[0] as i32 - pixel[0] as i32).abs();
            let b_diff = (alt_pixel[1] as i32 - pixel[1] as i32).abs();
            let g_diff = (alt_pixel[2] as i32 - pixel[2] as i32).abs();

            let difference = r_diff + g_diff + b_diff;
            
            // if difference != 0{
            //     println!("Difference {}", difference);
            // }
            

            if let Some(character) = find_character_by_value(alphabet, difference) {
                decrypted_message.push(character);
            }
        }
    }

    decrypted_message


}

fn find_character_by_value(alphabet: &HashMap<char, (i32, i32)>, value: i32) -> Option<char> {
    for (&character, &(a, _b)) in alphabet {
        if a == value {
            return Some(character);
        }
    }
    None
}