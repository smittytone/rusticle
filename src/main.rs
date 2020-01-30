//! An example of generating julia fractals.
extern crate image;
extern crate num_complex;


fn main() {
    let img_x = 4000;
    let img_y = 4000;

    let scale_x = 2.4 / img_x as f32;
    let scale_y = 2.4 / img_y as f32;

    let delta = 255.0 / img_x as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut img_buf = image::ImageBuffer::new(img_x, img_y);

    // Iterate over the coordinates and pixels of the image
    // This adds the red/blue background
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let r = (delta * y as f32) as u8;
        let b = (delta * x as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
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

    // Save the image as “fractal.png”, the format is deduced from the path
    img_buf.save("fractal.png").unwrap();
}