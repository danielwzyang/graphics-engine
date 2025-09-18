use std::fs::File;
use std::io::{self, Write};

pub struct Picture {
    xres: usize,
    yres: usize,
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

    pub fn save(&self, filename: &str) -> io::Result<()> {
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

        // everything went well return ok
        Ok(())
    }

    pub fn plot(&mut self, x: usize, y: usize, color: (usize, usize, usize)) -> io::Result<()> {
        // set color
        self.data[y][x] = color;

        // everthing went well
        Ok(())
    }
}
