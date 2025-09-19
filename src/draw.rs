use crate::picture::{Picture};
use std::io::{self};

pub fn line(picture: &mut Picture, x0: isize, y0: isize, x1: isize, y1: isize, color: &(usize, usize, usize)) -> io::Result<()> {
    let mut x = x0;
    let mut y = y0;
    let a = 2 * (y1 - y0);
    let b = -2 * (x1 - x0);
    let mut d = a + b/2;

    while x <= x1 {
        picture.plot(x as usize, y as usize, &color)?;
        
        if d > 0 {
            y += 1;
            d += b;
        }

        x += 1;
        d += a;
    }

    Ok(())
}
