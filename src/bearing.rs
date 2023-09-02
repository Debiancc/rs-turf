use geojson::{Feature, Position};
use std::f64;
use crate::helper::{degrees_to_radians, radians_to_degrees};

pub fn bearing(start: &Position, end: &Position, final_calc: bool) -> f64 {
    if final_calc {
        return calculate_final_bearing(start, end);
    }
    let lon1 = degrees_to_radians(start[0]);
    let lon2 = degrees_to_radians(end[0]);
    let lat1 = degrees_to_radians(start[1]);
    let lat2 = degrees_to_radians(end[1]);

    radians_to_degrees(
        (
            (lon2 - lon1).sin() * lat2.cos()
        )
            .atan2(
                lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * (lon2 - lon1).cos()
            )
    )
}

pub fn calculate_final_bearing(start: &Position, end: &Position) -> f64 {
    let bear = bearing(end, start, false);
    (bear + 180.) % 360.
}


#[cfg(test)]
pub(crate) mod tests {
    use super::*;


    #[test]
    fn bearing_test() {
        let x: Vec<_> = (0..10).collect();

        let start: &Position = &vec![-75., 45.];
        let end: &Position = &vec![20., 60.];

        assert_eq!(bearing(start, end, false), 37.75495852601734, "normal");
        assert_eq!(bearing(start, end, true), 120.01405215181424, "final case")
    }
}