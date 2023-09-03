use geojson::{LineStringType, Position};
use crate::bearing::bearing;
use crate::destination::destination;
use crate::distance::distance as distance_fn;
use crate::types::Units;

pub fn along(line: &LineStringType, distance: f64, u: Option<&Units>) -> Position {
    let mut travelled = 0.;
    for (i, position) in line.iter().enumerate() {
        if distance >= travelled && i == line.len() - 1 {
            break;
        } else if travelled >= distance {
            let overshot = distance - travelled;
            if overshot == 0. {
                return position.clone();
            } else {
                let direction = bearing(&line[i], &line[i - 1], false) - 180.;
                let interpolated = destination(
                    &line[i],
                    overshot,
                    direction,
                    u
                );
                return interpolated;
            }
        } else {
            travelled += distance_fn(&line[i], &line[i+1], u)
        }
    }

    let s = &line[line.len() - 1];
    s.clone()
}