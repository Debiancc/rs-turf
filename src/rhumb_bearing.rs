use std::f64::consts::PI;
use geojson::Position;
use crate::helper::{degrees_to_radians, radians_to_degrees};

fn calc(s: &Position, e: &Position) -> f64 {
    let phi1 = degrees_to_radians(e[1]);
    let phi2 = degrees_to_radians(s[1]);
    let mut delta_lambda = degrees_to_radians(e[0] - s[0]);

    if delta_lambda > PI {
        delta_lambda -= 2. * PI;
    }

    if delta_lambda < -PI {
        delta_lambda += 2. * PI;
    }

    let delta_psi = ((
        phi2 / 2. + PI / 4.
    ).tan() / (
        phi1 / 2. + PI / 4.
    ).tan()).log(10.);

    let theta = delta_lambda.atan2(delta_psi);
    (radians_to_degrees(theta) + 360.) % 360.
}

pub fn rhumb_bearing(s: &Position, e: &Position, is_final: bool) -> f64 {
    let mut bear360;
    if is_final {
        bear360 = calc(e, s);
    } else {
        bear360 = calc(s, e)
    }

    if bear360 > 180. {
        return -(360. - bear360);
    }
    bear360
}