use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::path::Path;

use image::{ImageBuffer, Rgb};
use show_image::{create_window, ImageInfo, ImageView, WindowOptions};

pub struct Picture {
    pub xres: usize,
    pub yres: usize,
    max_color: usize,
    data: Vec<Vec<(usize, usize, usize)>>,
    default_color: (usize, usize, usize),
    z_buffer: Vec<Vec<f32>>,
}

impl Picture {
    // constructor
    pub fn new(xres: usize, yres: usize, max_color: usize, default_color: &(usize, usize, usize)) -> Picture {
        // using vectors to save space + possibly unknown res at compile time
        let default_color = default_color.clone();
        let data = vec![vec![default_color.clone(); xres]; yres];
        let z_buffer = vec![vec![f32::NEG_INFINITY; xres]; yres];
        Picture {
            xres,
            yres,
            max_color,
            data,
            default_color,
            z_buffer,
        }
    }

    pub fn clear(&mut self) {
        // fill data with default color
        self.data = vec![vec![self.default_color.clone(); self.xres]; self.yres];
        // reset z-buffer as well so previous depth values don't interfere
        self.z_buffer = vec![vec![f32::NEG_INFINITY; self.xres]; self.yres];
    }

    pub fn display(&self) -> Result<(), Box<dyn Error>> {
        // create image buffer
        let mut buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(self.xres as u32, self.yres as u32);

        // fill buffer with pixels
        for (y, row) in self.data.iter().enumerate() {
            for (x, &(r, g, b)) in row.iter().enumerate() {
                let pixel = Rgb([r as u8, g as u8, b as u8]);
                buffer.put_pixel(x as u32, y as u32, pixel);
            }
        }

        // convert the buffer to raw bytes (RGB8 format)
        let bytes = buffer.into_raw();

        // create imageview
        let image = ImageView::new(
            ImageInfo::rgb8(self.xres as u32, self.yres as u32),
            &bytes,
        );

        // create window
        let window = create_window("Preview", WindowOptions {
            size: Some([self.xres as u32, self.yres as u32]),
            ..Default::default()
        })?;

        // use image buffer to set window
        window.set_image("image", image)?;

        // keep the window open until manually closed
        window.wait_until_destroyed()?;

        Ok(())
    }


    pub fn save_as_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let extension = Path::new(filename)
            .extension() // get extension
            .and_then(|s| s.to_str()) // convert to a string
            .unwrap_or("") // if invalid string then default to empty string
            .to_ascii_lowercase(); // to lowercase

        match extension.as_str() {
            "ppm" => {
                self.save_ppm(filename)?;
                println!("{} created.", filename);
                Ok(())
            }
            "png" => {
                self.save_png(filename)?;
                println!("{} created.", filename);
                Ok(())
            }
            "" => {
                return Err(format!("Cannot save file: please provide a file extension.").into());
            }
            _ => {
                return Err(format!("Cannot save file: .{} not supported.", extension).into());
            }
        }
    }

    fn save_ppm(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(filename)?;

        // ppm header
        writeln!(file, "P3 {} {} {}", self.xres, self.yres, self.max_color)?;

        // input rgb
        for row in &self.data {
            for &(r, g, b) in row {
                writeln!(file, "{} {} {}", r, g, b)?;
            }
        }

        Ok(())
    }

    fn save_png(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        // create image buffer
        let mut buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(self.xres as u32, self.yres as u32);

        // iterate through data and set the pixels in the image buffer
        for (y, row) in self.data.iter().enumerate() {
            for (x, &(r, g, b)) in row.iter().enumerate() {
                let pixel = Rgb([r as u8, g as u8, b as u8]);
                buffer.put_pixel(x as u32, y as u32, pixel);
            }
        }

        buffer.save(filename)?;

        Ok(())
    }

    fn plot(&mut self, x: usize, y: usize, z: f32, color: &(usize, usize, usize)) {
        // ignore pixels out of bounds
        if y >= self.yres || x >= self.xres {
            return;
        }

        let y = (self.yres - 1) - y;

        // z buffer
        if z < self.z_buffer[y][x] {
            return;
        }

        // flip the y coords so the origin is at the bottom left instead of top left
        self.data[y][x] = (color.0, color.1, color.2);
        self.z_buffer[y][x] = z;
    }

    pub fn draw_line(&mut self, mut x0: isize, mut y0: isize, mut z0: f32, x1: isize, y1: isize, z1: f32, color: &(usize, usize, usize)) {
        // using absolute value to make it case insensitive for the different octants
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        // if left to right then ++ otherwise --
        let step_x = if x1 > x0 { 1 } else { -1 };
        // if down to up then ++ otherwise --
        let step_y = if y1 > y0 { 1 } else { -1 };
        // ^^ these make it so it can move in multi directions

        // using dx and dy instead of y1 - y0 and x1 - x0 so it works for all octants
        let a = 2 * dy;
        let b = -2 * dx;

        // 0 <= |m| <= 1
        let small_slope = dy <= dx;

        // a and b derived from line equation Ax + By + C
        // imagine d as the cumulative error as we move through the line
        let mut d = if small_slope { a + b / 2 } else { b + a / 2 };

        if small_slope {
            // step in z = delta z / # of pixels plotted
            // for small slope it's delta x
            // for big slope its delta y
            let step_z = (z1 - z0) / (dx as f32 + 1.0);

            // there is at least one pixel for every x value for small slope
            loop {
                self.plot(x0 as usize, y0 as usize, z0, &color);

                if x0 == x1 {
                    break;
                }

                // the y value needs to be stepped when we "fall below" the line
                // it's not actually falling below the line for all octants,
                // but because we use absolute value it treats the case as if it were in octant 1
                // because b is a negative value, we only add b if d is positive to get it closer to 0
                if d > 0 {
                    y0 += step_y;
                    d += b;
                }

                // since |m| falls between 0 and 1 we know we always step x
                x0 += step_x;
                d += a;

                z0 += step_z;
            }
        } else {
            let step_z = (z1 - z0) / (dy as f32 + 1.0);

            // there is at least one pixel for every y value for big slope
            loop {
                self.plot(x0 as usize, y0 as usize, z0, &color);

                if y0 == y1 {
                    break;
                }

                // a similar idea here where the x value needs to be stepped if we are "on the left"
                // a is a positive value, so we only add a if d is negative to get it closer to 0
                if d < 0 {
                    x0 += step_x;
                    d += a;
                }

                // we always step y
                y0 += step_y;
                d += b;

                z0 += step_z;
            }
        }

        // plot last point
        self.plot(x0 as usize, y0 as usize, z0, &color);
    }
}
