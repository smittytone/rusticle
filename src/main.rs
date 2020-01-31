extern crate image;
extern crate num_complex;

use std::str::FromStr;



fn main() {

    // Get the command line arguments except the first
    let args: Vec<String> = std::env::args().collect();

    // TODO Clean up the arg processing

    // Parse the image size
    let size = parse_pair(&args[1], 'x');

    // Set the image dimensions
    let img_x = if size.0 == 0 { 4000 } else { size.0 };
    let img_y = if size.1 == 0 { 4000 } else { size.1 };

    // Get the output filename, or set a default
    let mut file_name = "render.png".to_string();
    if args.len() >= 3 { file_name = args[2].clone(); }

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut img_buf = image::ImageBuffer::new(img_x, img_y);

    // Iterate over the coordinates and pixels of the image
    // This adds the red/blue background
    let delta = 255.0 / img_x as f32;
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let r = (delta * y as f32) as u8;
        let b = (delta * x as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // julia(img_x, img_y, &img_buf);

    let scale_x = 2.4 / img_x as f32;
    let scale_y = 2.4 / img_y as f32;

    for x in 0..img_x {
        for y in 0..img_y {
            let cx = y as f32 * scale_x - 1.0;
            let cy = x as f32 * scale_y - 1.0;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = img_buf.get_pixel_mut(x, y);

            // Read RGB value of pixel
            let image::Rgb(data) = *pixel;

            // Set the green value of the pixel, preserving red and blue
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // If the filename doesn't contain '.png', add it
    match &file_name.find(".png") {
        None => { file_name.push_str(".png") },
        Some(_index) => {}
    };

    println!{"File: {}", file_name};

    // Write out the image buffer
    img_buf.save(&file_name).unwrap();
}


// Parse a pair of arguments into a tuple
fn parse_pair(string: &str, separator: char) -> (u32, u32) {

    match string.find(separator) {
        // Return usable 'nil' tuple on error, ie. no separator found
        None => { return (0, 0); }
        Some(index) => {
            // Decode the two strings either side of the separator into u32s,
            // and return as tuple if good, or (0, 0) if not
            match (u32::from_str(&string[..index]), u32::from_str(&string[index + 1..])) {
                (Ok(left_arg), Ok(right_arg)) => return { (left_arg, right_arg) },
                _ => { return (0, 0); }
            }
        }
    };
}

/*
fn julia<P: image::Pixel, C>(img_x: u32, img_y: u32, &img_buf: image::ImageBuffer<P, C>) {

    let scale_x = 2.4 / img_x as f32;
    let scale_y = 2.4 / img_y as f32;

    for x in 0..img_x {
        for y in 0..img_y {
            let cx = y as f32 * scale_x - 1.0;
            let cy = x as f32 * scale_y - 1.0;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = *img_buf.get_pixel_mut(x, y);

            // Read RGB value of pixel
            let image::Rgb(data) = *pixel;

            // Set the green value of the pixel, preserving red and blue
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }
}
*/