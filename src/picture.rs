use std::fs::File;
use std::io::{self, Write};

pub struct Picture {
    pub xres: usize,
    pub yres: usize,
    max_color: usize,
    data: Vec<Vec<(usize, usize, usize)>>,
}

impl Picture {
    // constructor
    pub fn new(xres: usize, yres: usize, max_color: usize) -> Picture {
        // using vectors to save space + possibly unknown res at compile time
        let data = vec![vec![(0, 0, 0); xres]; yres];
        Picture {
            xres,
            yres,
            max_color,
            data
        }
    }

    pub fn save_as_file(&self, filename: &str) -> io::Result<()> {
        // create file
        let mut pic_file = File::create(filename)?;

        // write header
        writeln!(pic_file, "P3 {} {} {}\n", self.xres, self.yres, self.max_color)?;

        // loop through data and write into file
        // take data as reference since we don't need ownership (just reading values)
        for row in &self.data {
            for &(r, g, b) in row {
                writeln!(pic_file, "{} {} {}\n", r, g, b)?;
            }
        }

        println!("Image file created: {}", filename);

        // everything went well return ok
        Ok(())
    }

    pub fn plot(&mut self, x: usize, y: usize, color: &(usize, usize, usize)) -> io::Result<()> {
        // set color
        self.data[(self.yres - 1) - y][x] = (color.0, color.1, color.2);

        // everthing went well
        Ok(())
    }

    pub fn draw_line(&mut self, mut x0: isize, mut y0: isize, x1: isize, y1: isize, color: &(usize, usize, usize)) -> io::Result<()> {
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

        // if |m| > 1 then it's as if we're swapping x and y (reflection over y = x)
        // a and b derived from line equation Ax + By + C
        let mut d = if small_slope { a + b / 2 } else { a / 2 + b };

        if small_slope {
            // there is at least one pixel for every x value for small slope
            while x0 != x1 {
                self.plot(x0 as usize, y0 as usize, &color)?;

                if d > 0 {
                    y0 += step_y;
                    d += b;
                }

                x0 += step_x;
                d += a;
            }
        } else {
            // there is at least one pixel for every y value for small slope
            while y0 != y1 {
                self.plot(x0 as usize, y0 as usize, &color)?;

                if d < 0 {
                    x0 += step_x;
                    d += a;
                }

                y0 += step_y;
                d += b;
            }
        }

        Ok(())
    }
}
