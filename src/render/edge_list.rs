use std::f32::consts::PI;

use crate::{
    constants::{PARAMETRIC_STEPS, HERMITE, BEZIER},
    matrix::add_point,
};
use super::Picture;

type EdgeList = Vec<[f32; 4]>;

pub fn add_edge(m: &mut EdgeList, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) {
    add_point(m, x0, y0, z0, 1.0);
    add_point(m, x1, y1, z1, 1.0);
}

pub fn render_edges(m: &EdgeList, picture: &mut Picture, color: &(usize, usize, usize)) {
    for edge in m.chunks(2) {
        // loop through in pairs
        picture.draw_line(edge[0][0] as isize, edge[0][1] as isize, edge[0][2], edge[1][0] as isize, edge[1][1] as isize, edge[1][2], &color);
    }
}

fn run_parametric<X, Y>(m: &mut EdgeList, x: X, y: Y, z: Option<f32>)
    where X: Fn(f32) -> f32, Y: Fn(f32) -> f32, {
    // we can use parametric equations for things like circles and splines
    // t = 0
    // t -> 1
    // x and y have their own functions of t
    let z_val = z.unwrap_or(0.0);

    // we need to store this so we can draw edges between consecutive points
    let mut last_point = (x(0.0), y(0.0), z_val);

    // t -> 1
    for i in 0..=PARAMETRIC_STEPS {
        let t = i as f32 / PARAMETRIC_STEPS as f32;
        let current_point = (x(t), y(t), z_val);

        add_edge(
            m,
            last_point.0, last_point.1, last_point.2,
            current_point.0, current_point.1, current_point.2,
        );

        last_point = current_point;
    }
}

pub fn add_circle(m: &mut EdgeList, cx: f32, cy: f32, cz: f32, r: f32) {
    // x(t) = rcos(2 * pi * t) + cx
    // y(t) = rsin(2 * pi * t) + cy
    let x = |t: f32| r * (2.0 * PI * t).cos() + cx;
    let y = |t: f32| r * (2.0 * PI * t).sin() + cy;

    run_parametric(m, x, y, Some(cz));
}

pub fn add_hermite_curve(m: &mut EdgeList, x0: f32, y0: f32, x1: f32, y1: f32, rx0: f32, ry0: f32, rx1: f32, ry1: f32) {
    // find coefficients for for at^3 + bt^2 + ct + d
    let mut g = vec![[x0, x1, rx0, rx1], [y0, y1, ry0, ry1]];
    crate::matrix::multiply(&HERMITE, &mut g);

    let x = |t: f32| t * (t * (t * g[0][0] + g[0][1]) + g[0][2]) + g[0][3];
    let y = |t: f32| t * (t * (t * g[1][0] + g[1][1]) + g[1][2]) + g[1][3];

    run_parametric(m, x, y, None);
}

pub fn add_bezier_curve(m: &mut EdgeList, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
    // find coefficients for for at^3 + bt^2 + ct + d
    let mut g = vec![[x0, x1, x2, x3], [y0, y1, y2, y3]];
    crate::matrix::multiply(&BEZIER, &mut g);

    let x = |t: f32| t * (t * (t * g[0][0] + g[0][1]) + g[0][2]) + g[0][3];
    let y = |t: f32| t * (t * (t * g[1][0] + g[1][1]) + g[1][2]) + g[1][3];

    run_parametric(m, x, y, None);
}
