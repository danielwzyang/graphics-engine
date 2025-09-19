use crate::picture::{Picture};
use std::io;
use std::mem;

pub fn line(picture: &mut Picture, mut x0: isize, mut y0: isize, mut x1: isize, mut y1: isize, color: &(usize, usize, usize)) -> io::Result<()> {
    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }

    let mut x = x0;
    let mut y = y0;
    let a = 2 * (y1 - y0);
    let b = -2 * (x1 - x0);

    let step_y = if y1 > y0 { 1 } else { -1 };

    // 0 <= m <= 1
    let is_slope_frac = (y1 - y0) <= (x1 - x0);

    let mut d = if is_slope_frac { a + b/2 } else { a/2 + b };

    if is_slope_frac {
        while x <= x1 {
            picture.plot(x as usize, y as usize, &color)?;

            if d > 0 {
                y += step_y;
                d += b;
            }

            x += 1;
            d += a;
        }
    } else {
        while y <= y1 {
            picture.plot(x as usize, y as usize, &color)?;

            if d < 0 {
                x += 1;
                d += a;
            }

            y += step_y;
            d += b;
        }
    }

    Ok(())
}
