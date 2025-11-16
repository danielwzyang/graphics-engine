type Vector = [f32; 3];

pub fn add_vectors(a: &Vector, b: &Vector) -> Vector {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn subtract_vectors(a: &Vector, b: &Vector) -> Vector {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn normalize_vector(vector: &Vector) -> Vector {
    let magnitude = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
    [vector[0] / magnitude, vector[1] / magnitude, vector[2] / magnitude]
}

pub fn cross_product(a: &Vector, b: &Vector) -> Vector {
    // < aybz - azby, azbx - axbz, axby - aybx >
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn dot_product(a: &Vector, b: &Vector) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}