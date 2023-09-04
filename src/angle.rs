use geojson::Position;
use crate::bearing::bearing;
use crate::helper::bearing_to_azimuth;
use crate::rhumb_bearing::rhumb_bearing;

fn calc_azimuth (
    start: &Position,
    end: &Position,
    mercator: bool
) -> f64 {
    if mercator {
        return bearing_to_azimuth(
            rhumb_bearing(start, end, false)
        )
    }
    return bearing_to_azimuth(
        bearing(start, end, false)
    )
}

pub fn angle(
    start: &Position,
    mid: &Position,
    end: &Position,
    explementary: bool,
    mercator: bool,
) -> f64 {
    let start_mid = calc_azimuth(start, mid, mercator);
    let end_mid = calc_azimuth(end, mid, mercator);
    let angle_start_mid = (start_mid - end_mid).abs();

    if explementary {
        return 360. - angle_start_mid
    }
    angle_start_mid
}


#[cfg(test)]
pub(crate) mod tests {
    use super::*;


    #[test]
    fn angle_test() {
        let x: Vec<_> = (0..10).collect();

        let start: &Position = &vec![-75., 45.];
        let end: &Position = &vec![20., 60.];

        assert_eq!(angle(&vec![5.,5.], &vec![5., 6.], &vec![3.,4.], false, false), 45., "45 degrees")
    }
}