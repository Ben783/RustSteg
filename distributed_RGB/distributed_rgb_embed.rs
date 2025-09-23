// image_matrix: 
// [
// [[r,g,b],[r,g,b],[r,g,b]...],
// [[r,g,b],[r,g,b],[r,g,b]...], 
//  ...]

use image::RgbImage;
use crate::simple_RGB::rgb_dictionary::get_rgb_dictionary;
use crate::simple_RGB::LSB_suitability::find_valid_modification;
use crate::matrix_conversion_RGB::matrix_conversion;
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


// fn split_image_matrix(image_matrix: Vec<Vec<[u8; 3]>>, pixel_dist: u8) -> Vec<Vec<Vec<[u8; 3]>>> {
//     let step = pixel_dist as usize; // pixel stepping distance
//     let h = image_matrix.len(); // height of the image matrix
//     let w = image_matrix[0].len(); // width of image matrix

//     let mut strips = Vec::new();

//     for col_start in (0..w).step_by(step) { // iterating over columns in steps of 'step'. This defines the starting pixels of each strip
//         let mut strip = Vec::new();
//         for row in &image_matrix { // iterating over rows in the image matrix (by reference)
//             let slice = row[col_start..(col_start + step).min(w)].to_vec(); // extracts a slice of the row starting from col_start to col_start + step.
//             strip.push(slice); // .min(w) ensures we stay in the counds of image width. That slice is converted to a vector.
//         }
//         strips.push(strip);
//     }

//     strips
// }



fn return_pixel_distributions(denary_rgb: Vec<u8>, n_pixels: u8) -> Vec<Vec<u8>>{ // distributes each denary value across a specified number of pixels.
    let mut distributions: Vec<Vec<u8>> = Vec::new();

    fn distribute_integer(change: u8, n_pixels: u8) -> Vec<u8>{ // distributing a change of m across n pixels.
        let mut distribution = Vec::new();

        let base = change / n_pixels; // Check if this is Rust for '//'
        let remainder = change % n_pixels;
        distribution = vec![base; n_pixels as usize];
        for i in 0..remainder{
            distribution[i as usize] += 1; // note that this line essentially just adds 1 to each pixel until the change is reached.
        } 

        distribution
    }

    for change in denary_rgb.iter(){
        distributions.push(distribute_integer(*change, n_pixels));
    }

    distributions // [[2,2,1], [2,3,2], ... ] if n = 3. changes of 5, 7, etc.

}



pub fn distributed_embedding(mut image_matrix: Vec<Vec<[u8; 3]>>, message: &str, pixel_dist: u8) -> Vec<Vec<[u8; 3]>>{

    let mut distributed_group_array = return_pixel_distributions(message_to_denary(message, &get_rgb_dictionary()), pixel_dist);  // [[2,2,1], [2,3,2], ... ] if n = 3. changes of 5, 7, etc.
    let mut group_index = 0; // setting a counter to track the group of denary modifications
    let message_length = distributed_group_array.len(); // message length
    let height = image_matrix.len(); // height of image matrix
    let width = image_matrix[0].len(); // width of image matrix

    for row in 0..height{ // iterating through each row
        for col_start in (0..width).step_by(pixel_dist as usize){
            if group_index < message_length && col_start + pixel_dist as usize <= width{// only alter FULL groups of pixels that are size pixel_dist
                let col_end = (col_start + pixel_dist as usize);
                let pixel_group = &mut image_matrix[row][col_start..col_end]; // gets a mutable slice of the pixel group in the image matrix
                println!("Original pixel group {:?}", pixel_group);
                let modification_group = &distributed_group_array[group_index]; // gets vector of denary mods for the pixels in this group
                let backup : Vec<[u8; 3]> = pixel_group.to_vec();
                let mut skip_group = false;
                for (i, pixel) in pixel_group.iter_mut().enumerate(){ // iterating through each pixel in the chunk
                    let [r, g, b] = *pixel;
                    let valid_modification = find_valid_modification(r, g, b, modification_group[i] as i32);
                    if valid_modification != [0, 0, 0]{
                        *pixel = valid_modification;
                    }
                    else{
                        skip_group = true;
                        break; // if a pixel in the group cannot be modified, entire group is skipped
                    }
                }

                if skip_group == true{ // if the group is skipped, revert all the pixels in the group to their original values.
                    for (pixel, backup_pixel) in pixel_group.iter_mut().zip(backup.iter()){ // pixel_group is a vector of mutable REFERENCES to a slice of the image matrix.
                        *pixel = *backup_pixel; // .zip() PAIRS each altered pixel with its backup counterpart, hence the (pixel, backup). 
                    }
                }
                else{
                    group_index += 1;
                    println!("Altered pixel group:{:?}", pixel_group);
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


pub fn complete_dist_embedding(image_matrix: Vec<Vec<[u8; 3]>>, message: &str, output_path: &str, pixel_dist: u8) -> Result<(), image:: ImageError>{
    let modified_matrix = distributed_embedding(image_matrix, message, pixel_dist);
    reconstruct_image(modified_matrix, output_path)
}


// for running independently in /bin...
// fn main() {
//     let org_image_path = r"C:\Users\HOME\RustSteg\Rust_Steg\images\TREE.png";
//     let output_path = r"C:\Users\HOME\RustSteg\Rust_Steg\images\dist_TREE.png";

//     let message = "aaa bbb ccc ddd";

//     let org_img_matrix = matrix_conversion(org_image_path);
//     let pixel_dist = 3;

//     let stego_matrix = distributed_embedding(org_img_matrix, message, pixel_dist);

//     reconstruct_image(stego_matrix, output_path)
//         .expect("Failed to save image");
// }
