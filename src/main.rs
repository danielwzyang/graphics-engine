mod picture;
mod draw;
mod colors;
use crate::picture::{Picture};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut test_pic = Picture::new(500, 500, 255);

    let xres = 500;
    let yres = 500;

    // octants 1, 5
    draw::line(&mut test_pic, 0, 0, xres-1, yres-1, &colors::GREEN)?;
    draw::line(&mut test_pic, 0, 0, xres-1, yres/2, &colors::GREEN)?;
    draw::line(&mut test_pic, xres-1, yres-1, 0, yres/2, &colors::GREEN)?;

    // octants 8, 4
    draw::line(&mut test_pic, 0, yres-1, xres-1, 0, &colors::CYAN)?;
    draw::line(&mut test_pic, 0, yres-1, xres-1, yres/2, &colors::CYAN)?;
    draw::line(&mut test_pic, xres-1, 0, 0, yres/2, &colors::CYAN)?;

    // octants 2, 6
    draw::line(&mut test_pic, 0, 0, xres/2, yres-1, &colors::RED)?;
    draw::line(&mut test_pic, xres-1, yres-1, xres/2, 0, &colors::RED)?;

    // octants 7, 3
    draw::line(&mut test_pic, 0, yres-1, xres/2, 0, &colors::MAGENTA)?;
    draw::line(&mut test_pic, xres-1, 0, xres/2, yres-1, &colors::MAGENTA)?;

    // horizontal and vertical
    draw::line(&mut test_pic, 0, yres/2, xres-1, yres/2, &colors::YELLOW)?;
    draw::line(&mut test_pic, xres/2, 0, xres/2, yres-1, &colors::YELLOW)?;

    test_pic.save("test.ppm")?;

    Ok(())
}
