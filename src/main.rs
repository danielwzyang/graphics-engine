mod picture;
mod draw;
mod colors;
use crate::picture::{Picture};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
   let mut pic = Picture::new(500, 500, 255);

   draw::line(&mut pic, 0, 0, 200, 150, &colors::GREEN)?;

   pic.save("image.ppm")?;

   Ok(())
}
