mod picture;
mod colors;
use crate::picture::{Picture};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut test_pic = Picture::new(500, 500, 255);

    let xres = 500;
    let yres = 500;

    // octants 1, 5
    test_pic.draw_line(0, 0, xres-1, yres-1, &colors::GREEN)?;
    test_pic.draw_line(0, 0, xres-1, yres/2, &colors::GREEN)?;
    test_pic.draw_line(xres-1, yres-1, 0, yres/2, &colors::GREEN)?;

    // octants 8, 4
    test_pic.draw_line(0, yres-1, xres-1, 0, &colors::CYAN)?;
    test_pic.draw_line(0, yres-1, xres-1, yres/2, &colors::CYAN)?;
    test_pic.draw_line(xres-1, 0, 0, yres/2, &colors::CYAN)?;

    // octants 2, 6
    test_pic.draw_line(0, 0, xres/2, yres-1, &colors::RED)?;
    test_pic.draw_line(xres-1, yres-1, xres/2, 0, &colors::RED)?;

    // octants 7, 3
    test_pic.draw_line(0, yres-1, xres/2, 0, &colors::MAGENTA)?;
    test_pic.draw_line(xres-1, 0, xres/2, yres-1, &colors::MAGENTA)?;

    // horizontal and vertical
    test_pic.draw_line(0, yres/2, xres-1, yres/2, &colors::YELLOW)?;
    test_pic.draw_line(xres/2, 0, xres/2, yres-1, &colors::YELLOW)?;

    test_pic.save("test.ppm")?;

    Ok(())
}
