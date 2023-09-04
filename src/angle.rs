use geojson::Position;
use crate::helper::bearing_to_azimuth;

fn calc_azimuth (
    start: &Position,
    end: &Position,
    mercator: bool
) -> f64 {
    if mercator {
        return bearing_to_azimuth(
            
        )
    }
}

pub fn angle(
    start: &Position,
    mid: &Position,
    end: &Position,
    explementary: bool,
    mercator: bool,
) -> f64 {
    if !mercator {
        
    }
}