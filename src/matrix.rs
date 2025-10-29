#![allow(dead_code)]

type Matrix = Vec<[f32; 4]>;

use std::f32::consts::PI;
use crate::picture::Picture;

pub enum Rotation {
    X,
    Y,
    Z,
}

const STEPS: i32 = 10;

// cubic hermite and bezier matrices to find polynomial coefficients
const HERMITE: [[f32; 4]; 4] = [
    [2.0, -3.0, 0.0, 1.0],
    [-2.0, 3.0, 0.0, 0.0],
    [1.0, -2.0, 1.0, 0.0],
    [1.0, -1.0, 0.0, 0.0],
];
const BEZIER: [[f32; 4]; 4] = [
    [-1.0, 3.0, -3.0, 1.0],
    [3.0, -6.0, 3.0, 0.0],
    [-3.0, 3.0, 0.0, 0.0],
    [1.0, 0.0, 0.0, 0.0],
];

pub fn new() -> Matrix {
    vec![]
}

pub fn identity() -> Matrix {
    let mut data = vec![[0.0, 0.0, 0.0, 0.0]; 4];

    // matrix of 0s with 1s at principal diagonal
    for (i, point) in data.iter_mut().enumerate() {
        (*point)[i] = 1.0;
    }

    data
}

pub fn multiply(m1: &[[f32; 4]], m2: &mut Matrix) {
    let mut data = vec![[0.0, 0.0, 0.0, 0.0]; m2.len()];

    // iterate through every point
    for (i, point) in data.iter_mut().enumerate() {
        // iterate through the items in the point (x, y, z, w)
        for j in 0..4 {
            // iterate through the rows of m1 and columns of m2
            for k in 0..4 {
                (*point)[j] += m1[k][j] * m2[i][k];
            }
        }
    }

    *m2 = data;
}

pub fn print(m1: &Matrix) {
    for i in 0..4 {
        for point in m1 {
            // iterate through and have 3 characters for each so it's evenly spaced (left-aligned)
            print!("{:^4}", point[i]);
        }

        println!()
    }
}

fn add_point(m: &mut Matrix, x: f32, y: f32, z: f32, w: f32) {
    m.push([x, y, z, w]);
}

pub fn add_edge(m: &mut Matrix, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) {
    add_point(m, x0, y0, z0, 1.0);
    add_point(m, x1, y1, z1, 1.0);
}

pub fn render_edges(m: &Matrix, picture: &mut Picture, color: &(usize, usize, usize)) {
    for edge in m.chunks(2) {
        // loop through in pairs
        picture.draw_line(edge[0][0] as isize, edge[0][1] as isize, edge[1][0] as isize, edge[1][1] as isize, &color);
    }
}

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

pub fn translate(m: &mut Matrix, a: f32, b: f32, c: f32) {
    let mut transformation_matrix = identity();

    // x, y, and z of last point are a, b, and c
    /*
        1 0 0 a
        0 1 0 b
        0 0 1 c
        0 0 0 1
    */

    transformation_matrix[3][0] = a;
    transformation_matrix[3][1] = b;
    transformation_matrix[3][2] = c;

    multiply(&transformation_matrix, m)
}

pub fn dilate(m: &mut Matrix, a: f32, b: f32, c: f32) {
    let mut transformation_matrix = identity();

    // 1s are replaced by a, b, and c
    /*
        a 0 0 0
        0 b 0 0
        0 0 c 0
        0 0 0 1
    */

    transformation_matrix[0][0] = a;
    transformation_matrix[1][1] = b;
    transformation_matrix[2][2] = c;

    multiply(&transformation_matrix, m)
}

pub fn rotate(m: &mut Matrix, axis: Rotation, degrees: f32) {
    let mut transformation_matrix = identity();
    let theta = degrees * (PI / 180.0);

    match axis {
        Rotation::Z => {
            /*
                cosθ -sinθ 0 0
                sinθ cosθ 0 0
                0 0 1 0
                0 0 0 1
            */

            transformation_matrix[0][0] = theta.cos();
            transformation_matrix[0][1] = theta.sin();
            transformation_matrix[1][0] = -theta.sin();
            transformation_matrix[1][1] = theta.cos();
        }

        Rotation::X => {
            /*
                1 0 0 0
                0 cosθ -sinθ 0
                0 sinθ cosθ 0
                0 0 0 1
            */

            transformation_matrix[1][1] = theta.cos();
            transformation_matrix[1][2] = theta.sin();
            transformation_matrix[2][1] = -theta.sin();
            transformation_matrix[2][2] = theta.cos();
        }

        Rotation::Y => {
            /*
                cosθ 0 sinθ 0
                0 1 0 0
                -sinθ 0 cosθ 0
                0 0 0 1
            */

            transformation_matrix[0][0] = theta.cos();
            transformation_matrix[0][2] = -theta.sin();
            transformation_matrix[2][0] = theta.sin();
            transformation_matrix[2][2] = theta.cos();
        }
    }

    multiply(&transformation_matrix, m)
}

fn run_parametric<X, Y>(m: &mut Matrix, x: X, y: Y, z: Option<f32>)
    where X: Fn(f32) -> f32, Y: Fn(f32) -> f32, {
    // we can use parametric equations for things like circles and splines
    // t = 0
    // t -> 1
    // x and y have their own functions of t
    let z_val = z.unwrap_or(0.0);

    // we need to store this so we can draw edges between consecutive points
    let mut last_point = (x(0.0), y(0.0), z_val);

    // t -> 1
    for i in 0..=STEPS {
        let t = i as f32 / STEPS as f32;
        let current_point = (x(t), y(t), z_val);

        add_edge(
            m,
            last_point.0, last_point.1, last_point.2,
            current_point.0, current_point.1, current_point.2,
        );

        last_point = current_point;
    }
}

pub fn add_circle(m: &mut Matrix, cx: f32, cy: f32, cz: f32, r: f32) {
    // x(t) = rcos(2 * pi * t) + cx
    // y(t) = rsin(2 * pi * t) + cy
    let x = |t: f32| r * (2.0 * PI * t).cos() + cx;
    let y = |t: f32| r * (2.0 * PI * t).sin() + cy;

    run_parametric(m, x, y, Some(cz));
}

pub fn add_hermite_curve(m: &mut Matrix, x0: f32, y0: f32, x1: f32, y1: f32, rx0: f32, ry0: f32, rx1: f32, ry1: f32) {
    // find coefficients for for at^3 + bt^2 + ct + d
    let mut g = vec![[x0, x1, rx0, rx1], [y0, y1, ry0, ry1]];
    multiply(&HERMITE, &mut g);

    let x = |t: f32| t * (t * (t * g[0][0] + g[0][1]) + g[0][2]) + g[0][3];
    let y = |t: f32| t * (t * (t * g[1][0] + g[1][1]) + g[1][2]) + g[1][3];

    run_parametric(m, x, y, None);
}

pub fn add_bezier_curve(m: &mut Matrix, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
    // find coefficients for for at^3 + bt^2 + ct + d
    let mut g = vec![[x0, x1, x2, x3], [y0, y1, y2, y3]];
    multiply(&BEZIER, &mut g);

    let x = |t: f32| t * (t * (t * g[0][0] + g[0][1]) + g[0][2]) + g[0][3];
    let y = |t: f32| t * (t * (t * g[1][0] + g[1][1]) + g[1][2]) + g[1][3];

    run_parametric(m, x, y, None);
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

    // 021
    add_polygon(m, 
        vertices[0][0], vertices[0][1], vertices[0][2], 
        vertices[2][0], vertices[2][1], vertices[2][2], 
        vertices[1][0], vertices[1][1], vertices[1][2],
    );

    // 032
    add_polygon(m, 
        vertices[0][0], vertices[0][1], vertices[0][2], 
        vertices[3][0], vertices[3][1], vertices[3][2], 
        vertices[2][0], vertices[2][1], vertices[2][2],
    );

    // 415
    add_polygon(m, 
        vertices[4][0], vertices[4][1], vertices[4][2], 
        vertices[1][0], vertices[1][1], vertices[1][2], 
        vertices[5][0], vertices[5][1], vertices[5][2],
    );

    // 401
    add_polygon(m, 
        vertices[4][0], vertices[4][1], vertices[4][2], 
        vertices[0][0], vertices[0][1], vertices[0][2], 
        vertices[1][0], vertices[1][1], vertices[1][2],
    );

    // 704
    add_polygon(m, 
        vertices[7][0], vertices[7][1], vertices[7][2], 
        vertices[0][0], vertices[0][1], vertices[0][2], 
        vertices[4][0], vertices[4][1], vertices[4][2],
    );

    // 730
    add_polygon(m, 
        vertices[7][0], vertices[7][1], vertices[7][2], 
        vertices[3][0], vertices[3][1], vertices[3][2], 
        vertices[0][0], vertices[0][1], vertices[0][2],
    );

    // 637
    add_polygon(m, 
        vertices[6][0], vertices[6][1], vertices[6][2], 
        vertices[3][0], vertices[3][1], vertices[3][2], 
        vertices[7][0], vertices[7][1], vertices[7][2],
    );

    // 623
    add_polygon(m, 
        vertices[6][0], vertices[6][1], vertices[6][2], 
        vertices[2][0], vertices[2][1], vertices[2][2], 
        vertices[3][0], vertices[3][1], vertices[3][2],
    );

    // 526
    add_polygon(m, 
        vertices[5][0], vertices[5][1], vertices[5][2], 
        vertices[2][0], vertices[2][1], vertices[2][2], 
        vertices[6][0], vertices[6][1], vertices[6][2],
    );

    // 512
    add_polygon(m, 
        vertices[5][0], vertices[5][1], vertices[5][2], 
        vertices[1][0], vertices[1][1], vertices[1][2], 
        vertices[2][0], vertices[2][1], vertices[2][2],
    );

    // 756
    add_polygon(m, 
        vertices[7][0], vertices[7][1], vertices[7][2], 
        vertices[5][0], vertices[5][1], vertices[5][2], 
        vertices[6][0], vertices[6][1], vertices[6][2],
    );

    // 745
    add_polygon(m, 
        vertices[7][0], vertices[7][1], vertices[7][2], 
        vertices[4][0], vertices[4][1], vertices[4][2], 
        vertices[5][0], vertices[5][1], vertices[5][2],
    );    
}

fn draw_points(m: &mut Matrix, points: Vec<[f32; 3]>) {
    // basically just draw a tiny line to make a point
    for point in points {
        add_edge(
            m,
            point[0], point[1], point[2],
            point[0], point[1], point[2], 
        )
    }
}

pub fn add_sphere(m: &mut Matrix, cx: f32, cy: f32, cz: f32, r: f32) {
    let points = generate_sphere_points(cx, cy, cz, r);
    println!("{:?}", points);

    // e.g. a STEPS of 10 results in 11 points per semicircle
    let steps = STEPS + 1;
    
    let get = |longitude: i32, latitude: i32| -> [f32; 3] {
        points[(longitude * steps + latitude) as usize]
    };

    for longitude in 0..steps {
        let next = if longitude + 1 == steps { 0 } else { longitude + 1 };
        // this is for all the polygons that aren't on the poles
        for latitude in 1..steps-1 {
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
        let pole = get(longitude, steps - 1);
        let p = get(longitude, steps - 2);
        let p_across = get(next, steps - 2);
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
    draw_points(m, points);
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
