mod picture;
use crate::picture::{Picture, PictureParameters};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
   let mut pic = Picture::new(PictureParameters {
       xres: 500,
       yres: 500,
       max_color: 255,
   });

   for x in 0..500 {
       for y in 0..500 {
           pic.plot(x, y, (x % 256, y % 256, 255))?;
       }
   }

   pic.save("image.ppm")?;

   Ok(())
}
