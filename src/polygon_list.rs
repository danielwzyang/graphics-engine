type Matrix = Vec<[f32; 4]>;

use std::f32::consts::PI;
use crate::picture::Picture;
use crate::constants::{STEPS, CUBE};
use crate::matrix::add_point;

pub fn add_polygon(m: &mut Matrix, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) {
    add_point(m, x0, y0, z0, 1.0);
    add_point(m, x1, y1, z1, 1.0);
    add_point(m, x2, y2, z2, 1.0);
}

pub fn render_polygons(m: &Matrix, picture: &mut Picture, color: &(usize, usize, usize)) {
    for edge in m.chunks(3) {
        picture.draw_line(edge[0][0] as isize, edge[0][1] as isize, edge[1][0] as isize, edge[1][1] as isize, &color);
        picture.draw_line(edge[1][0] as isize, edge[1][1] as isize, edge[2][0] as isize, edge[2][1] as isize, &color);
        picture.draw_line(edge[2][0] as isize, edge[2][1] as isize, edge[0][0] as isize, edge[0][1] as isize, &color);
    }
}

pub fn add_box(m: &mut Matrix, x: f32, y: f32, z: f32, w: f32, h: f32, d: f32) {
    /*
        4 ---- 5
      / |    / |
    0 ---- 1   | h
    |   |  |   |
    |   7 -|-- 6
    | /    | /  d
    3 ---- 2
       w
    */

    let vertices = [
        [x, y, z],
        [x + w, y, z],
        [x + w, y - h, z],
        [x, y - h, z],
        [x, y, z - d],
        [x + w, y, z - d],
        [x + w, y - h, z - d],
        [x, y - h, z - d],
    ];

    for polygon in CUBE {
        let (a, b, c) = polygon;
        add_polygon(m,
            vertices[a][0], vertices[a][1], vertices[a][2],
            vertices[b][0], vertices[b][1], vertices[b][2],
            vertices[c][0], vertices[c][1], vertices[c][2],
        );
    }
}

pub fn add_sphere(m: &mut Matrix, cx: f32, cy: f32, cz: f32, r: f32) {
    let points = generate_sphere_points(cx, cy, cz, r);

    // we do STEPS + 1 because the semicircle generates one extra point for the south pole the way I coded it
    // e.g. a STEPS of 10 results in 11 points per semicircle

    let get = |longitude: i32, latitude: i32| -> [f32; 3] {
        points[(longitude * STEPS + 1 + latitude) as usize]
    };

    for longitude in 0..=STEPS {
        let next = if longitude == STEPS { 0 } else { longitude + 1 };
        // this is for all the polygons that aren't on the poles
        for latitude in 1..STEPS {
            let p1 = get(longitude, latitude);
            let p2 = get(longitude, latitude + 1);
            let p1_across = get(next, latitude);
            let p2_across = get(next, latitude + 1);

            // p1, p2, p2_across
            add_polygon(m,
                p1[0], p1[1], p1[2],
                p2[0], p2[1], p2[2],
                p2_across[0], p2_across[1], p2_across[2],
            );

            // p1, p2_across, p1_across
            add_polygon(m,
                p1[0], p1[1], p1[2],
                p2_across[0], p2_across[1], p2_across[2],
                p1_across[0], p1_across[1], p1_across[2],
            );
        }

        // two triangles at the poles

        // pole, p1, p1_across
        let pole = get(longitude, 0);
        let p = get(longitude, 1);
        let p_across = get(next, 1);
        add_polygon(m,
            pole[0], pole[1], pole[2],
            p[0], p[1], p[2],
            p_across[0], p_across[1], p_across[2],
        );

        // pole, pminus1_across, pminus1
        let pole = get(longitude, STEPS);
        let p = get(longitude, STEPS - 1);
        let p_across = get(next, STEPS - 1);
        add_polygon(m,
            pole[0], pole[1], pole[2],
            p_across[0], p_across[1], p_across[2],
            p[0], p[1], p[2],
        );
    }
}

fn generate_sphere_points(cx: f32, cy: f32, cz: f32, r: f32) -> Vec<[f32; 3]> {
    // not using run_parametric because this parametric is nested but the logic is the same
    let x = |cir: f32| r * (PI * cir).cos() + cx;
    let y = |rot: f32, cir: f32| r * (PI * cir).sin() * (2.0 * PI * rot).cos() + cy;
    let z = |rot: f32, cir: f32| r * (PI * cir).sin() * (2.0 * PI * rot).sin() + cz;

    let mut point_list: Vec<[f32; 3]> = vec![];

    for i in 0..=STEPS {
        let rot = i as f32 / STEPS as f32;
        for j in 0..=STEPS {
            let cir = j as f32 / STEPS as f32;
            point_list.push([x(cir), y(rot, cir), z(rot, cir)]);
        }
    }

    point_list
}

pub fn add_torus(m: &mut Matrix, cx: f32, cy: f32, cz: f32, r1: f32, r2: f32) {
    let points = generate_torus_points(cx, cy, cz, r1, r2);

    // around is which circle of the torus we're currently on
    // on is which part of the circle we're currently on
    // kind of silly names but longitude and latitude didn't make sense so i had to freestyle it
    // for the torus we can just use STEPS i.e. STEPS of 10 gives 10 points on each circle
    let get = |around: i32, on: i32| -> [f32; 3] {
        points[(around * STEPS + on) as usize]
    };

    for around in 0..=STEPS {
        let next = if around == STEPS { 0 } else { around + 1 };
        for on in 0..=STEPS {
            let p1 = get(around, on);
            let p2 = get(around, on + 1);
            let p1_across = get(next, on);
            let p2_across = get(next, on + 1);

            // p1, p2, p2_across
            add_polygon(m,
                p1[0], p1[1], p1[2],
                p2[0], p2[1], p2[2],
                p2_across[0], p2_across[1], p2_across[2],
            );

            // p1, p2_across, p1_across
            add_polygon(m,
                p1[0], p1[1], p1[2],
                p2_across[0], p2_across[1], p2_across[2],
                p1_across[0], p1_across[1], p1_across[2],
            );
        }
    }
}

fn generate_torus_points(cx: f32, cy: f32, cz: f32, r1: f32, r2: f32) -> Vec<[f32; 3]> {
    // r1 is the radius of the circle that makes up the torus
    // r2 is the radius of the entire torus (translation factor)
    let x = |rot: f32, cir: f32| (2.0 * PI * rot).cos() * (r1 * (2.0 * PI * cir).cos() + r2) + cx;
    let y = |cir: f32| r1 * (2.0 * PI * cir).sin() + cy;
    let z = |rot: f32, cir: f32| -1.0 * (2.0 * PI * rot).sin() * (r1 * (2.0 * PI * cir).cos() + r2) + cz;

    let mut point_list: Vec<[f32; 3]> = vec![];

    for i in 0..=STEPS {
        let rot = i as f32 / STEPS as f32;
        for j in 0..=STEPS {
            let cir = j as f32 / STEPS as f32;
            point_list.push([x(rot, cir), y(cir), z(rot, cir)]);
        }
    }

    point_list
}
