use geojson::{GeoJson, PointType};
use crate::bbox::bbox;

pub fn center(geojson: &GeoJson) -> PointType {
    let ext = bbox(geojson, false);
    let x = (ext[0] + ext[2]) / 2.;
    let y = (ext[1] + ext[3]) / 2.;
    vec![x, y]
}