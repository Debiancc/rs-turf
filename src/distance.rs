use geojson::Position;
use crate::helper::{degrees_to_radians, radians_to_degrees, radians_to_length};
use crate::types::Units;

pub fn distance(from: &Position, to: &Position, units: Option<Units>) -> f64 {
    let d_lat = degrees_to_radians(to[1] - from[1]);
    let d_lon = degrees_to_radians(to[0] - from[0]);
    let lat1 = degrees_to_radians(from[1]);
    let lat2 = degrees_to_radians(to[1]);

    let a = (d_lat / 2.).sin().powi(2) +
        (d_lon / 2.).sin().powi(2) * lat1.cos() * lat2.cos();

    radians_to_length(
        2. * a.sqrt().atan2((1. - a).sqrt()),
        units,
    )
}