use geojson::{GeoJson, PointType};
use crate::bbox::bbox;
use crate::coord_each::coord_each;

pub fn center(geojson: &GeoJson) -> PointType {
    let ext = bbox(geojson, false);
    let x = (ext[0] + ext[2]) / 2.;
    let y = (ext[1] + ext[3]) / 2.;
    vec![x, y]
}

pub fn centerid(geojson: &GeoJson) -> PointType {
    let mut x = 0_f64;
    let mut y = 0_f64;
    let mut len = 0_f64;

    coord_each(geojson, |coord, _, _, _, _| {
        x += coord[0];
        y += coord[1];
        len += 1.;
        true
    }, false);

    let x = x / len;
    let y = y / len;
    return vec![x, y]
}