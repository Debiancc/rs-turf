use geojson::{Position};
use crate::helper::{degrees_to_radians, length_to_radians, radians_to_degrees};
use crate::types::Units;

pub fn destination(origin: &Position, distance: f64, bearing: f64, unit: Option<Units>) -> Position {
    let long1 = degrees_to_radians(origin[0]);
    let lat1 = degrees_to_radians(origin[1]);
    let bearing_rad = degrees_to_radians(bearing);
    let radians = length_to_radians(distance, unit);

    let lat2 = (
        lat1.sin() * radians.cos() + lat1.cos() * radians.sin() * bearing_rad.cos()
    ).atan();
    let long2 = long1 + (
        bearing_rad.sin() * radians.sin() * lat1.cos()
    ).atan2(
        radians.cos() - lat1.sin() * lat2.sin()
    );

    return vec![
        radians_to_degrees(long2),
        radians_to_degrees(lat2),
    ];
}