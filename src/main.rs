mod picture;
use crate::picture::{Picture, PictureParameters};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
   let mut pic = Picture::new(PictureParameters {
       xres: 500,
       yres: 500,
       max_color: 255,
   });

   let center = (250.0, 250.0);
   let rays = 10.0;

   for x in 0..500 {
       for y in 0..500 {
           let dx = x as f32 - center.0;
           let dy = y as f32 - center.1;
           let dist = (dx*dx + dy*dy).sqrt() / 10.0;
           let angle = dy.atan2(dx) * rays;
           let warped = angle * 6.0 + dist.sin() * 3.0;

           let r = (warped.sin() * 127.0 + 128.0) as usize;
           let g = ((warped + 2.0).sin() * 127.0 + 128.0) as usize;
           let b = ((warped + 4.0).sin() * 127.0 + 128.0) as usize;

           pic.plot(x, y, (r, g, b))?;
       }
   }

   pic.save("image.ppm")?;

   Ok(())
}
