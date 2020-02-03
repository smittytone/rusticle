extern crate image;
extern crate num_complex;

use image::{ImageBuffer, Rgb};
use std::str::FromStr;


struct JuliaSet {
    image_buf: image::ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl JuliaSet {

    pub fn new(width: u32, height: u32) -> JuliaSet {

        // Create a new buffer
        let mut image_buf = ImageBuffer::new(width, height);

        // Set the background: sweep of red and blue colour values
        let delta = 255.0 / width as f32;
        for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
            let r = (delta * y as f32) as u8;
            let b = (delta * x as f32) as u8;
            *pixel = image::Rgb([r, 0, b]);
        }

        // Return a new JuliaSet with the image buffer
        JuliaSet { image_buf: image_buf }
    }

    pub fn render(&mut self) {

        // Set the image scaling
        let scale_x = 2.4 / self.image_buf.width() as f32;
        let scale_y = 2.4 / self.image_buf.height() as f32;

        for x in 0..self.image_buf.width() {
            for y in 0..self.image_buf.height() {
                // Generate  the number of iterations for a given pixel
                let cx = y as f32 * scale_x - 1.0;
                let cy = x as f32 * scale_y - 1.0;

                let c = num_complex::Complex::new(-0.4, 0.6);
                let mut z = num_complex::Complex::new(cx, cy);

                let mut i = 0;
                while i < 255 && z.norm() <= 2.0 {
                    z = z * z + c;
                    i += 1;
                }

                // Read the current RGB value of the pixel
                let pixel = self.image_buf.get_pixel_mut(x, y);
                let Rgb(data) = *pixel;

                // Write the RGB value back, adding the green value, and
                // preserving the red and blue background values
                *pixel = Rgb([data[0], i as u8, data[2]]);
            }
        }
    }

    pub fn save(&self, file_name: String) {
        // Output the image to disk
        self.image_buf.save(file_name).expect("[ERROR] Could not save image file");
    }
}


fn main() {

    // Get the command line arguments except the first
    let args: Vec<String> = std::env::args().collect();

    // TODO Clean up the arg processing

    // Parse the image size
    let size = parse_pair(&args[1], 'x');

    // Set the image dimensions
    let img_width = if size.0 == 0 { 4000 } else { size.0 };
    let img_height = if size.1 == 0 { 4000 } else { size.1 };

    // Get the output filename, or set a default
    let mut file_name = "render.png".to_string();
    if args.len() >= 3 { file_name = args[2].clone(); }

    // If the filename doesn't contain '.png', add it
    match &file_name.find(".png") {
        None => { file_name.push_str(".png") },
        Some(_index) => {}
    };

    println!("Rendered image size: {}x{}", img_width, img_height);
    println!{"File: {}", file_name};

    // Generate the Julia Set
    let mut julia_set = JuliaSet::new(img_width, img_height);
    julia_set.render();

    // Write out the image buffer
    julia_set.save(file_name);
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
