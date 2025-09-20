// matrix (mxn):
//    [
//    [[r,g,b],[r,g,b],[r,g,b], ...],
//    [[r,g,b],[r,g,b],[r,g,b], ...],
//    ...
//    ]


use image::ImageReader;

pub fn matrix_conversion(image_path: &str) -> Vec<Vec<[u8; 3]>>{
    let img = ImageReader::open(image_path) // opening selected image and declaring it as a tuple (u32, u32)
    .expect("Failed to open file")
    .decode()
    .expect("Failed to decode image")
    .to_rgb8(); // <-- convert to RgbImage (RGB ONLY. no alpha)

    let (width, height) = img.dimensions(); // obtaining image dimensions
    let raw_pixels = img.into_raw(); // converts image into a flat Vec<u8> [R, G, B, R, G, B, ...]

    // Convert to 2D matrix: Vec<Vec<[u8; 3]>>
    let mut matrix: Vec<Vec<[u8; 3]>> = Vec::with_capacity(height as usize); // Declaring a vector OF vectors (2D array), with each inner array being composed of 3 integer values.
    // ^^^ also referencing variables as usize is safer. Reserves memory for height rows in advance.
    
    // Process the raw pixels in chunks of (width * 3) bytes (one row, 'chunked' out of the flat array). One 'row_chunk' is all the RGB values in the row (3*width, as width counts pixels, not channels.)
    for row_chunk in raw_pixels.chunks_exact((width * 3) as usize) {
        let mut row: Vec<[u8; 3]> = Vec::with_capacity(width as usize); // For each height ROW, a new vector is created, with each element being [u8; 3]. There are 'width' amount of pixels. 
        
        // Process each pixel (3 bytes: R, G, B)
        for pixel_chunk in row_chunk.chunks_exact(3) { // 'Chunks' the rows down into chunks of 3. Each chunk of three is a 'pixel_chunk'.
            let pixel = [
                pixel_chunk[0], // R
                pixel_chunk[1], // G
                pixel_chunk[2], // B
            ];
            row.push(pixel); // Pushing pixels consisting of 3 values onto the row
        }
        matrix.push(row); // Pushing the row onto the matrix, to obtain a matrix the same dimensions as the original image. [[[R,g,b], [r,g,b], ...]]
    }

    println!("Pixel (0,0): {:?}", matrix[0][0]); // First pixel RGB
    println!("Pixel (10,20): {:?}", matrix[20][10]); // Pixel at (x=10, y=20)

    // println!(matrix)

    matrix
}