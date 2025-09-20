
//  denary_rgb:
//  [1,3,6,2,3,5, ...]


//  array_dist:
//  [[r,g,b],[r,g,b],[r,g,b], ...] 


use image::RgbImage;
use super::rgb_dictionary::get_rgb_dictionary;
use super::LSB_suitability::find_valid_modification;
use std::collections::HashMap;

fn message_to_denary(message: &str, dictionary: &HashMap<char, (i32, i32)>) -> Vec<u8>{

    let mut denary_RGB : Vec<u8> = Vec::new(); // Creating a mutable vector to store the denary RGB modification values
    let fallback : char = '?'; // IMPORTANT: when declaring a char, character must be encapsulated in '', NOT ""

    for character in message.chars(){ // for each character in the input string...
        match dictionary.get(&character){ // Look up character in the dictionary. If there is 'some' match, push RGB value. If there is no match, display error.
            Some(&(a, b)) => denary_RGB.push(a as u8), // comma used at the end here as the code is not encapsulated in {}
            None => {
                // println!("Character not found: {}", character);
                denary_RGB.push(fallback as u8) // Pushing the fallback character's ASCII denary value
            }

        }
    }

    denary_RGB // [2, 4, 2, 7, 3, ...]
}


pub fn simple_seq_embedding(mut image_matrix: Vec<Vec<[u8; 3]>>, message: &str) -> Vec<Vec<[u8; 3]>>{

    let denary_array = message_to_denary(message, &get_rgb_dictionary());
    let mut denary_index = 0;
    let message_length = denary_array.len();

    for row in image_matrix.iter_mut(){ // 
        for pixel in row.iter_mut(){ // 
            let [r, g, b] = *pixel;
            let mut skip_flag = false;
            if denary_index < message_length{
                println!("Original RGB: {},{},{}", r, g, b);
                let valid_modification = find_valid_modification(r, g, b, denary_array[denary_index] as i32);
                if valid_modification == [0, 0, 0]{
                    skip_flag = true; // skips this pixel, tries to embed in the next pixel
                }
                if skip_flag == false{
                    *pixel = valid_modification;
                    denary_index += 1;
                    println!("Alt RGB: {:?}", *pixel);
                }
            }
            else{
                break;
            }
        }
    }
    image_matrix
}


pub fn reconstruct_image(matrix: Vec<Vec<[u8; 3]>>, output_path: &str) -> Result<(), image::ImageError>{
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

pub fn complete_simpleRGB(image_matrix: Vec<Vec<[u8; 3]>>, message: &str, output_path: &str) -> Result<(), image:: ImageError>{
    let modified_matrix = simple_seq_embedding(image_matrix, message);
    reconstruct_image(modified_matrix, output_path)
}





// fn optimal_3_channel_dist (denary_rgb: Vec<u8>) -> Vec<[u8; 3]>{ // Returning a vector of arrays, each array has 3 unsigned 8 bit integers (Vec<[u8, 3]>)
//     let mut array_dist = Vec::new(); // declaring a mutable vector
//     for number in denary_rgb.iter(){
//         let number = *number;
//         let mut r = 0;
//         let mut g = 0;
//         let mut b = 0;
//         if number % 3 == 0{
//             r = number/3;
//             g = number/3;
//             b = number/3;
//         }
//         else if number % 3 == 1{
//             r = (number-1)/3; 
//             g = (number-1)/3;
//             b = (number-1)/3 +1;
//         }
//         else if number % 3 == 2{
//             r = (number+1)/3;
//             g = (number+1)/3;
//             b = (number+1)/3 -1;
//         }
//         array_dist.push([r,g,b]);
//     }
    
//     array_dist

// }


// pub fn smart_add(channel_val: u8, dist_val: u8) -> u8{
//     let new_val = channel_val as u16 + dist_val as u16;
//     if new_val>255{
//         channel_val.saturating_sub(dist_val)
//     }
//     else{
//         new_val as u8
//     }
    
// }