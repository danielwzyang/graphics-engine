use crate::picture::Picture;
use std::io;

pub fn line(picture: &mut Picture, mut x0: isize, mut y0: isize, x1: isize, y1: isize, color: &(usize, usize, usize)) -> io::Result<()> {
    // using absolute value to make it case insensitive for the different octants
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    // if left to right then ++ otherwise --
    let step_x = if x1 > x0 { 1 } else { -1 };
    // if down to up then ++ otherwise --
    let step_y = if y1 > y0 { 1 } else { -1 };
    // ^^ these make it so 

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
            picture.plot(x0 as usize, y0 as usize, &color)?;

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
            picture.plot(x0 as usize, y0 as usize, &color)?;

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
