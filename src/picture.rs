use std::{
    fs::File,
    io::Write,
    error::Error,
    path::Path,
};

use crate::constants::ENABLE_Z_BUFFER;
use image::{ImageBuffer, Rgb};
use show_image::{create_window, ImageInfo, ImageView, WindowOptions};

pub struct Picture {
    pub xres: usize,
    pub yres: usize,
    max_color: usize,
    pub data: Vec<u8>, // flat rgb buffer that stores as [r, g, b, r, g, b, ...]
    default_color: (usize, usize, usize),
    z_buffer: Vec<Vec<f32>>,
}

impl Picture {
    pub fn new(xres: usize, yres: usize, max_color: usize, default_color: &(usize, usize, usize)) -> Self {
        let default_color = default_color.clone();
        let mut data = vec![0; xres * yres * 3];
        for y in 0..yres {
            for x in 0..xres {
                let i = (y * xres + x) * 3;
                data[i] = default_color.0 as u8;
                data[i + 1] = default_color.1 as u8;
                data[i + 2] = default_color.2 as u8;
            }
        }

        let z_buffer = vec![vec![f32::NEG_INFINITY; xres]; yres];

        Self {
            xres,
            yres,
            max_color,
            data,
            default_color,
            z_buffer,
        }
    }

    pub fn clear(&mut self) {
        for y in 0..self.yres {
            for x in 0..self.xres {
                let i = (y * self.xres + x) * 3;
                self.data[i] = self.default_color.0 as u8;
                self.data[i + 1] = self.default_color.1 as u8;
                self.data[i + 2] = self.default_color.2 as u8;
            }
        }

        self.z_buffer = vec![vec![f32::NEG_INFINITY; self.xres]; self.yres];
    }

    pub fn display(&self) -> Result<(), Box<dyn Error>> {
        let image = ImageView::new(
            ImageInfo::rgb8(self.xres as u32, self.yres as u32),
            &self.data,
        );

        let window = create_window("Preview", WindowOptions {
            size: Some([self.xres as u32, self.yres as u32]),
            ..Default::default()
        })?;

        window.set_image("image", image)?;
        window.wait_until_destroyed()?;

        Ok(())
    }

    pub fn save_as_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let extension = Path::new(filename)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

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
            "" => Err("Cannot save file: please provide a file extension.".into()),
            _ => Err(format!("Cannot save file: .{} not supported.", extension).into()),
        }
    }

    fn save_ppm(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(filename)?;
        writeln!(file, "P3 {} {} {}", self.xres, self.yres, self.max_color)?;

        for y in 0..self.yres {
            for x in 0..self.xres {
                let i = (y * self.xres + x) * 3;
                writeln!(file, "{} {} {}", self.data[i], self.data[i+1], self.data[i+2])?;
            }
        }

        Ok(())
    }

    fn save_png(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(self.xres as u32, self.yres as u32, self.data.clone())
            .ok_or("Failed to create image buffer")?;

        buffer.save(filename)?;
        Ok(())
    }

    pub fn plot(&mut self, x: isize, y: isize, z: f32, color: &(usize, usize, usize)) {
        if x < 0 || y < 0 || x >= self.xres as isize || y >= self.yres as isize {
            return;
        }

        let x = x as usize;
        let y = y as usize;
        let y = (self.yres - 1) - y;

        let z_truncated = (z * 10000.0) as isize;
        let buffer_truncated = (self.z_buffer[y][x] * 10000.0) as isize;

        if ENABLE_Z_BUFFER && z_truncated < buffer_truncated {
            return;
        }

        let i = (y * self.xres + x) * 3;
        self.data[i] = color.0 as u8;
        self.data[i + 1] = color.1 as u8;
        self.data[i + 2] = color.2 as u8;

        self.z_buffer[y][x] = z;
    }

    pub fn draw_line(&mut self, mut x0: isize, mut y0: isize, mut z0: f32, x1: isize, y1: isize, z1: f32, color: &(usize, usize, usize)) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let step_x = if x1 > x0 { 1 } else { -1 };
        let step_y = if y1 > y0 { 1 } else { -1 };
        let a = 2 * dy;
        let b = -2 * dx;

        let small_slope = dy <= dx;
        let mut d = if small_slope { a + b / 2 } else { b + a / 2 };

        if small_slope {
            let step_z = (z1 - z0) / (dx as f32 + 1.0);
            loop {
                self.plot(x0, y0, z0, &color);
                if x0 == x1 { break; }
                if d > 0 {
                    y0 += step_y;
                    d += b;
                }
                x0 += step_x;
                d += a;
                z0 += step_z;
            }
        } else {
            let step_z = (z1 - z0) / (dy as f32 + 1.0);
            loop {
                self.plot(x0, y0, z0, &color);
                if y0 == y1 { break; }
                if d < 0 {
                    x0 += step_x;
                    d += a;
                }
                y0 += step_y;
                d += b;
                z0 += step_z;
            }
        }

        self.plot(x0, y0, z0, &color);
    }
}
