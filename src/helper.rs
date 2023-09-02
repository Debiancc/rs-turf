use std::f64::consts::PI;

pub fn radians_to_degrees(radians: f64) -> f64 {
    let degrees = radians % (2. * PI);
    (degrees * 180.) / PI
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    let radians = degrees % 360.;
    (radians * PI) / 180.
}