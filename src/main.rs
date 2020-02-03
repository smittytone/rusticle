/*
 * IMPORTS
 *
 */
extern crate image;
extern crate num_complex;

use image::{ImageBuffer, Rgb};
use std::str::FromStr;


/*
 * CONSTANTS
 *
 */
const TYPE_JULIA_SET: u8 = 0;
const TYPE_MANDEL_SET: u8 = 1;


/*
 * STRUCTS
 *
 */

/*
 * Simple struct for command line args
 */
struct Args {
    image_size: (u32, u32),
    image_name: String,
    image_type: u8,
    debug: bool
}


/*
 * Simple struct to hold drawing data
 */
struct Scales {
    height: u32,
    width: u32,
    scale_x: f32,
    scale_y: f32,
    x_offset: u32,
    y_offset: u32
}


/*
 * A struct for rendering and saving various fractal sets
 */
struct Set {
    image_buf: image::ImageBuffer<Rgb<u8>, Vec<u8>>,
    set_type: u8,
    debug: bool
}

impl Set {

    pub fn new(width: u32, height: u32, mut set_type: u8, debug: bool) -> Set {

        // Check the type -- probably a better way of doing this
        if set_type != TYPE_JULIA_SET && set_type != TYPE_MANDEL_SET {
            set_type = TYPE_JULIA_SET;
        }

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
        Set { image_buf: image_buf,
              set_type: set_type,
              debug: debug }
    }

    pub fn render(&mut self) {

        // Select the correct renderer for the specified set type
        match self.set_type {
            TYPE_MANDEL_SET =>  { self.render_mandel(); }
            _ =>                { self.render_julia(); }
        }
    }

    pub fn save(&self, file_name: String) {

        // Output the image to disk
        self.image_buf.save(file_name).expect("[ERROR] Could not save image file");
    }

    fn render_julia(&mut self) {

        // Render the Julia Set
        let scales = self.centre_image();
        if self.debug { println!("Rendering Julia Set @ {}x{}", self.image_buf.width(), self.image_buf.height()); }

        for x in 0..scales.width {
            for y in 0..scales.height {
                // Generate  the number of iterations for a given pixel
                let cx = y as f32 * scales.scale_x - 1.0;
                let cy = x as f32 * scales.scale_y - 1.0;

                let c = num_complex::Complex::new(-0.4, 0.6);
                let mut z = num_complex::Complex::new(cx, cy);

                let mut i = 0;
                while i < 255 && z.norm() <= 2.0 {
                    z = z * z + c;
                    i += 1;
                }

                // Read the current RGB value of the pixel
                let pixel = self.image_buf.get_pixel_mut(x + scales.x_offset, y + scales.y_offset);
                let Rgb(data) = *pixel;

                // Write the RGB value back, adding the green value, and
                // preserving the red and blue background values
                *pixel = Rgb([data[0], i as u8, data[2]]);
            }
        }
    }

    fn render_mandel(&mut self) {

        // Render the Mandelbrot Set
        let scales = self.centre_image();
        if self.debug { println!("Rendering Mandelbrot Set @ {}x{}", self.image_buf.width(), self.image_buf.height()); }

        let x_delta = 1.54; // 70% of width
        let y_delta = (scales.height as f32 / 2.0) * scales.scale_y;

        for x in 0..scales.width {
            for y in 0..scales.height {
                // Generate  the number of iterations for a given pixel
                let cx = x as f32 * scales.scale_x - x_delta;
                let cy = y as f32 * scales.scale_y - y_delta;

                let c = num_complex::Complex::new(cx, cy);
                let mut z = num_complex::Complex::new(0.0, 0.0);

                let mut i = 0;
                while i < 255 && z.norm_sqr() <= 4.0 {
                    z = z * z + c;
                    i += 1;
                }

                // Read the current RGB value of the pixel
                let pixel = self.image_buf.get_pixel_mut(x + scales.x_offset, y + scales.y_offset);
                let Rgb(data) = *pixel;

                // Write the RGB value back, adding the green value, and
                // preserving the red and blue background values
                *pixel = Rgb([data[0], i as u8, data[2]]);
            }
        }
    }

    fn centre_image(&mut self) -> Scales {
        // Render into a square in the centre of the window
        let mut width: u32 = self.image_buf.width();
        let mut height: u32 = self.image_buf.height();
        let mut x_offset: u32 = 0;
        let mut y_offset: u32 = 0;

        if width > height {
            x_offset = (width - height) / 2;
            width = height;
        } else if height > width {
            y_offset = (height - width) / 2;
            height = width;
        }

        // Set the image scaling
        let scale_base = if self.set_type == TYPE_JULIA_SET { 2.4 } else { 2.2 };
        let scale_x = scale_base / width as f32;
        let scale_y = scale_base / height as f32;

        // Return the drawing values
        Scales { width: width,
                 height: height,
                 scale_x: scale_x,
                 scale_y: scale_y,
                 x_offset: x_offset,
                 y_offset: y_offset }
    }
}


/*
 * RUNTIME START
 *
 */

fn main() {

    // Get the command line arguments except the first
    let input = parse_args();

    // Generate the chosen Set
    let mut set = Set::new(input.image_size.0,
                           input.image_size.1,
                           input.image_type,
                           input.debug);
    set.render();

    // Write out the image buffer
    set.save(input.image_name);
}


/*
 * Parse the command line arguments
 */
fn parse_args() -> Args {

    // Get the command line arguments except the first
    let mut args: Vec<String> = std::env::args().collect();

    // Set the defaults
    let mut values = Args { image_size: (400, 400),
                            image_type: TYPE_MANDEL_SET,
                            image_name: "fractal.png".to_string(),
                            debug: false };

    let mut is_value = false;
    let mut arg_type: i32 = -1;

    for i in 1..args.len() {
        let arg = &mut args[i];
        if is_value {
            is_value = false;

            match arg_type {
                0 => {
                    let size = parse_pair(arg, 'x');
                    let width = if size.0 == 0 { 400 } else { size.0 };
                    let height = if size.1 == 0 { 400 } else { size.1 };
                    values.image_size = (width, height);
                }
                1 => {
                    values.image_type = match u8::from_str(arg) {
                        Ok(value) => value,
                        Err(_err) => {
                            println!("[ERROR] invalid type value ({})", arg);
                            std::process::exit(1);
                        }
                    };
                }
                2 => {
                    let mut arg_string = arg.to_string();
                    match arg_string.find(".png") {
                        None => { arg_string.push_str(".png") },
                        Some(_index) => {}
                    };

                    values.image_name = arg_string;
                }
                _ => {
                    println!("[ERROR] bad option argument ({})", arg);
                    std::process::exit(1);
                }
            }
        } else {
            let arg_string = arg.to_string().to_lowercase();
            match &arg_string[..] {
                "-s" | "--size" => {
                    // Set the
                    is_value = true;
                    arg_type = 0;
                }
                "-t" | "--type" => {
                    is_value = true;
                    arg_type = 1;
                }
                "-o" | "--out" => {
                    is_value = true;
                    arg_type = 2;
                }
                "-v" | "--verbose" => {
                    values.debug = true;
                }
                _ => {
                    println!("[ERROR] unknown option ({})", arg_string);
                    std::process::exit(1);
                }
            }
        }
    }

    values
}


/*
 * Extract two values in a multiplex argument, eg. 567x789 into a
 * two-value tuple
 */
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
