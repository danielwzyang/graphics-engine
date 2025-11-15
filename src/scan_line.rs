use crate::picture::Picture;
type Vector = [f32; 3];
use crate::lighting::{LightingConfig, ReflectionConstants, get_illumination};

// i'm aware there's a lot of repeating code for all 3 functions,
// but i wanted each function to be more readable so bear with me
// (also i was quite confused implementing so i needed to make them separate to understand it myself)

pub fn flat(
    picture: &mut Picture, 
    polygon: &[[f32; 4]], 
    color: &(usize, usize, usize)
) {
    let p0 = polygon[0];
    let p1 = polygon[1];
    let p2 = polygon[2];

    // sort three points by their y values so we have a bottom top and middle
    let mut b = [p0[0], p0[1], p0[2]];
    let mut m = [p1[0], p1[1], p1[2]];
    let mut t = [p2[0], p2[1], p2[2]];

    if b[1] > m[1] {
        std::mem::swap(&mut b, &mut m);
    }
    if m[1] > t[1] {
        std::mem::swap(&mut m, &mut t);
    }
    if b[1] > m[1] {
        std::mem::swap(&mut b, &mut m);
    }

    /*
        scan line conversion works by drawing a bunch of horizontal lines to fill in the polygon
        lets imagine triangle BMT

            T

                    M

                B

        as our horizontal lines move up from b, we need to figure out a delta x on each side to adjust our endpoints
        in this case, the left side has a constant delta x which we will call dx0
        on the right side, BM and MT have different slopes, so we will call them dx1 and dx1_1 respectively
        we also do the same for the z values

        dx0 = (xt - xb) / (yt - yb)
        dx1 = (xm - xb) / (ym - yb)
        dx1_1 = (xt - xm) / (yt - ym)

        dz0 = (zt - zb) / (yt - yb)
        dz1 = (zm - zb) / (ym - yb)
        dz1_1 = (zt - zm) / (yt - ym)

        we do have to be careful with triangles that have flat tops and flat bottoms though
        honestly i'm not entirely sure what i did that fixed it, but i think it has to do with my flip boolean
        before that my flat bottom triangles would have a weird NaN slope on the right side

        also something REALLY annoying is cumulative floating point error which ends up with a lot of weird artifacts and gaps

        to fix this i kind of spam integer conversion
    */

    let y_start = b[1] as isize;
    let y_mid = m[1] as isize;
    let y_end = t[1] as isize;

    let distance0 = (y_end - y_start) as f32 + 1.0;
    let distance1 = (y_mid - y_start) as f32 + 1.0;
    let distance2 = (y_end - y_mid) as f32 + 1.0;

    let dx0 = if distance0 != 0.0 { (t[0] - b[0]) / distance0 } else { 0.0 };
    let dz0 = if distance0 != 0.0 { (t[2] - b[2]) / distance0 } else { 0.0 };
    let mut dx1 = if distance1 != 0.0 { (m[0] - b[0]) / distance1 } else { 0.0 };
    let mut dz1 = if distance1 != 0.0 { (m[2] - b[2]) / distance1 } else { 0.0 };

    let mut x0 = b[0];
    let mut z0 = b[2];
    let mut x1 = b[0];
    let mut z1 = b[2];

    let mut flip = false;
    let mut y = y_start;

    while y <= y_end {
        // switch slopes if we mass the middle
        if !flip && y >= y_mid {
            flip = true;
            dx1 = if distance2 != 0.0 { (t[0] - m[0]) / distance2 } else { 0.0 };
            dz1 = if distance2 != 0.0 { (t[2] - m[2]) / distance2 } else { 0.0 };
            x1 = m[0];
            z1 = m[2];
        }

        picture.draw_line(
            x0 as isize,
            y,
            z0,
            x1 as isize,
            y,
            z1,
            &color,
        );

        // increment
        x0 += dx0;
        z0 += dz0;
        x1 += dx1;
        z1 += dz1;
        y += 1;
    }
}

pub fn gouraud(
    picture: &mut Picture, 
    polygon: &[[f32; 4]], 
    normals: [Vector; 3],
    lighting_config: &LightingConfig,
    reflection_constants: &ReflectionConstants,
) {
    let p0 = polygon[0];
    let p1 = polygon[1];
    let p2 = polygon[2];
    
    let mut b = [p0[0], p0[1], p0[2]];
    let mut m = [p1[0], p1[1], p1[2]];
    let mut t = [p2[0], p2[1], p2[2]];
    
    // we need to sort the colors too
    // the difference between this and phong is that phong we will interpolate by normals instead of colors
    let mut color_b = get_illumination(&normals[0], lighting_config, reflection_constants);
    let mut color_m = get_illumination(&normals[1], lighting_config, reflection_constants);
    let mut color_t = get_illumination(&normals[2], lighting_config, reflection_constants);
    
    if b[1] > m[1] {
        std::mem::swap(&mut b, &mut m);
        std::mem::swap(&mut color_b, &mut color_m);
    }
    if m[1] > t[1] {
        std::mem::swap(&mut m, &mut t);
        std::mem::swap(&mut color_m, &mut color_t);
    }
    if b[1] > m[1] {
        std::mem::swap(&mut b, &mut m);
        std::mem::swap(&mut color_b, &mut color_m);
    }
    
    let y_start = b[1] as usize;
    let y_mid = m[1] as usize;
    let y_end = t[1] as usize;

    let distance0 = (y_end - y_start) as f32 + 1.0;
    let distance1 = (y_mid - y_start) as f32 + 1.0;
    let distance2 = (y_end - y_mid) as f32 + 1.0;
    
    let dx0 = if distance0 != 0.0 { (t[0] - b[0]) / distance0 } else { 0.0 };
    let dz0 = if distance0 != 0.0 { (t[2] - b[2]) / distance0 } else { 0.0 };
    let mut dx1 = if distance1 != 0.0 { (m[0] - b[0]) / distance1 } else { 0.0 };
    let mut dz1 = if distance1 != 0.0 { (m[2] - b[2]) / distance1 } else { 0.0 };
    
    // we need to do the same calculations for colors as we did for the coordinates
    let dr0 = if distance0 != 0.0 { (color_t.0 as f32 - color_b.0 as f32) / distance0 } else { 0.0 };
    let dg0 = if distance0 != 0.0 { (color_t.1 as f32 - color_b.1 as f32) / distance0 } else { 0.0 };
    let db0 = if distance0 != 0.0 { (color_t.2 as f32 - color_b.2 as f32) / distance0 } else { 0.0 };
    
    let mut dr1 = if distance1 != 0.0 { (color_m.0 as f32 - color_b.0 as f32) / distance1 } else { 0.0 };
    let mut dg1 = if distance1 != 0.0 { (color_m.1 as f32 - color_b.1 as f32) / distance1 } else { 0.0 };
    let mut db1 = if distance1 != 0.0 { (color_m.2 as f32 - color_b.2 as f32) / distance1 } else { 0.0 };
    
    let mut x0 = b[0];
    let mut z0 = b[2];
    let mut x1 = b[0];
    let mut z1 = b[2];
    
    let mut r0 = color_b.0 as f32;
    let mut g0 = color_b.1 as f32;
    let mut b0 = color_b.2 as f32;
    let mut r1 = color_b.0 as f32;
    let mut g1 = color_b.1 as f32;
    let mut b1 = color_b.2 as f32;
    
    let mut flip = false;
    let mut y = y_start;
    
    while y <= y_end {
        if !flip && y >= y_mid {
            flip = true;
            dx1 = if distance2 != 0.0 { (t[0] - m[0]) / distance2 } else { 0.0 };
            dz1 = if distance2 != 0.0 { (t[2] - m[2]) / distance2 } else { 0.0 };
            x1 = m[0];
            z1 = m[2];
            
            dr1 = if distance2 != 0.0 { (color_t.0 as f32 - color_m.0 as f32) / distance2 } else { 0.0 };
            dg1 = if distance2 != 0.0 { (color_t.1 as f32 - color_m.1 as f32) / distance2 } else { 0.0 };
            db1 = if distance2 != 0.0 { (color_t.2 as f32 - color_m.2 as f32) / distance2 } else { 0.0 };
            
            r1 = color_m.0 as f32;
            g1 = color_m.1 as f32;
            b1 = color_m.2 as f32;
        }
        
        // we want to draw our horizontal lines but we can't use draw_line because every pixel in the line is different
        let mut x_start = x0 as usize;
        let mut x_end = x1 as usize;
        let mut z_start = z0;
        let mut z_end = z1;
        let mut cr_start = r0;
        let mut cg_start = g0;
        let mut cb_start = b0;
        let mut cr_end = r1;
        let mut cg_end = g1;
        let mut cb_end = b1;
        
        // make sure we go left to right since we're going to use a for loop
        if x_start > x_end {
            std::mem::swap(&mut x_start, &mut x_end);
            std::mem::swap(&mut z_start, &mut z_end);
            std::mem::swap(&mut cr_start, &mut cr_end);
            std::mem::swap(&mut cg_start, &mut cg_end);
            std::mem::swap(&mut cb_start, &mut cb_end);
        }
        
        // calculate our steps for each pixel
        let distance = (x_end - x_start) as f32 + 1.0;
        let dz = if distance != 0.0 { (z_end - z_start) / distance } else { 0.0 };
        let dcr = if distance != 0.0 { (cr_end - cr_start) / distance } else { 0.0 };
        let dcg = if distance != 0.0 { (cg_end - cg_start) / distance } else { 0.0 };
        let dcb = if distance != 0.0 { (cb_end - cb_start) / distance } else { 0.0 };
        
        // start drawing the line!
        let mut z = z_start;
        let mut cr = cr_start;
        let mut cg = cg_start;
        let mut cb = cb_start;
        
        for x in x_start..=x_end {
            // we use our interpolated colors instead of calculating the color at every pixel
            // again we use plot not draw_line because every color is different
            picture.plot(x, y, z, &(
                cr.clamp(0.0, 255.0) as usize,
                cg.clamp(0.0, 255.0) as usize,
                cb.clamp(0.0, 255.0) as usize,
            ));
            
            z += dz;
            cr += dcr;
            cg += dcg;
            cb += dcb;
        }
        
        // increment
        x0 += dx0;
        z0 += dz0;
        x1 += dx1;
        z1 += dz1;
        
        r0 += dr0;
        g0 += dg0;
        b0 += db0;
        r1 += dr1;
        g1 += dg1;
        b1 += db1;
        
        y += 1;
    }
}

pub fn phong(
    picture: &mut Picture, 
    polygon: &[[f32; 4]], 
    normals: [Vector; 3],
    lighting_config: &LightingConfig,
    reflection_constants: &ReflectionConstants,
) {
    let p0 = polygon[0];
    let p1 = polygon[1];
    let p2 = polygon[2];
    
    let mut b = [p0[0], p0[1], p0[2]];
    let mut m = [p1[0], p1[1], p1[2]];
    let mut t = [p2[0], p2[1], p2[2]];
    
    // as i said earlier, phong is different than gouraud in that it interpolates by normals
    // here were don't calculate the color, we will do that at every pixel
    let mut n_b = normals[0];
    let mut n_m = normals[1];
    let mut n_t = normals[2];
    
    if b[1] > m[1] {
        std::mem::swap(&mut b, &mut m);
        std::mem::swap(&mut n_b, &mut n_m);
    }
    if m[1] > t[1] {
        std::mem::swap(&mut m, &mut t);
        std::mem::swap(&mut n_m, &mut n_t);
    }
    if b[1] > m[1] {
        std::mem::swap(&mut b, &mut m);
        std::mem::swap(&mut n_b, &mut n_m);
    }
    
    let y_start = b[1] as usize;
    let y_mid = m[1] as usize;
    let y_end = t[1] as usize;
    let distance0 = (y_end - y_start) as f32 + 1.0;
    let distance1 = (y_mid - y_start) as f32 + 1.0;
    let distance2 = (y_end - y_mid) as f32 + 1.0;
    
    let dx0 = if distance0 != 0.0 { (t[0] - b[0]) / distance0 } else { 0.0 };
    let dz0 = if distance0 != 0.0 { (t[2] - b[2]) / distance0 } else { 0.0 };
    let mut dx1 = if distance1 != 0.0 { (m[0] - b[0]) / distance1 } else { 0.0 };
    let mut dz1 = if distance1 != 0.0 { (m[2] - b[2]) / distance1 } else { 0.0 };
    
    // interpolate our normals
    let dnx0 = if distance0 != 0.0 { (n_t[0] - n_b[0]) / distance0 } else { 0.0 };
    let dny0 = if distance0 != 0.0 { (n_t[1] - n_b[1]) / distance0 } else { 0.0 };
    let dnz0 = if distance0 != 0.0 { (n_t[2] - n_b[2]) / distance0 } else { 0.0 };
    
    let mut dnx1 = if distance1 != 0.0 { (n_m[0] - n_b[0]) / distance1 } else { 0.0 };
    let mut dny1 = if distance1 != 0.0 { (n_m[1] - n_b[1]) / distance1 } else { 0.0 };
    let mut dnz1 = if distance1 != 0.0 { (n_m[2] - n_b[2]) / distance1 } else { 0.0 };
    
    let mut x0 = b[0];
    let mut z0 = b[2];
    let mut x1 = b[0];
    let mut z1 = b[2];
    
    let mut nx0 = n_b[0];
    let mut ny0 = n_b[1];
    let mut nz0 = n_b[2];
    let mut nx1 = n_b[0];
    let mut ny1 = n_b[1];
    let mut nz1 = n_b[2];
    
    let mut flip = false;
    let mut y = y_start;
    
    while y <= y_end {
        if !flip && y >= y_mid {
            flip = true;
            dx1 = if distance2 != 0.0 { (t[0] - m[0]) / distance2 } else { 0.0 };
            dz1 = if distance2 != 0.0 { (t[2] - m[2]) / distance2 } else { 0.0 };
            x1 = m[0];
            z1 = m[2];
            
            dnx1 = if distance2 != 0.0 { (n_t[0] - n_m[0]) / distance2 } else { 0.0 };
            dny1 = if distance2 != 0.0 { (n_t[1] - n_m[1]) / distance2 } else { 0.0 };
            dnz1 = if distance2 != 0.0 { (n_t[2] - n_m[2]) / distance2 } else { 0.0 };
            
            nx1 = n_m[0];
            ny1 = n_m[1];
            nz1 = n_m[2];
        }
        
        // once again draw our horizontal lines: same procedure
        let mut x_start = x0 as usize;
        let mut x_end = x1 as usize;
        let mut z_start = z0;
        let mut z_end = z1;
        let mut nx_start = nx0;
        let mut ny_start = ny0;
        let mut nz_start = nz0;
        let mut nx_end = nx1;
        let mut ny_end = ny1;
        let mut nz_end = nz1;
        
        if x_start > x_end {
            std::mem::swap(&mut x_start, &mut x_end);
            std::mem::swap(&mut z_start, &mut z_end);
            std::mem::swap(&mut nx_start, &mut nx_end);
            std::mem::swap(&mut ny_start, &mut ny_end);
            std::mem::swap(&mut nz_start, &mut nz_end);
        }
        
        let distance = (x_end - x_start) as f32 + 1.0;
        let dz = if distance != 0.0 { (z_end - z_start) / distance } else { 0.0 };
        let dnx = if distance != 0.0 { (nx_end - nx_start) / distance } else { 0.0 };
        let dny = if distance != 0.0 { (ny_end - ny_start) / distance } else { 0.0 };
        let dnz = if distance != 0.0 { (nz_end - nz_start) / distance } else { 0.0 };
        
        let mut z = z_start;
        let mut nx = nx_start;
        let mut ny = ny_start;
        let mut nz = nz_start;
        
        for x in x_start..=x_end {
            // this time we compute light based on our interpolated normal
            picture.plot(x, y, z, &get_illumination(&[nx, ny, nz], lighting_config, reflection_constants));
            
            z += dz;
            nx += dnx;
            ny += dny;
            nz += dnz;
        }
        
        x0 += dx0;
        z0 += dz0;
        x1 += dx1;
        z1 += dz1;
        
        nx0 += dnx0;
        ny0 += dny0;
        nz0 += dnz0;
        nx1 += dnx1;
        ny1 += dny1;
        nz1 += dnz1;
        
        y += 1;
    }
}
