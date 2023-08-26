// use geojson:{F }

use geojson::{Feature, GeoJson, Position, Value};

fn each(coord: Vec<Position>) {}

// currentCoord, coordIndex, featureIndex, multiFeatureIndex, geometryIndex

type Callback = fn(current_coord: Position, coord_index: usize, feature_index: usize, multi_feature_index: usize, geometry_index: usize) -> bool;

fn e(features: Vec<Feature>, cb: Callback, exclude_wrap_coord: bool) {
    let mut coord_index: usize = 0;
    let mut multi_feature_index: usize = 0;
    let mut geometry_index: usize = 0;

    for (feature_index, feature) in features.iter().enumerate() {
        if feature.geometry.is_some() {
            match feature.geometry.unwrap() {
                Value::Point(p) => {
                    if cb(p, coord_index, feature_index, multi_feature_index, geometry_index) == false {
                        return;
                    }
                    coord_index += 1;
                    multi_feature_index += 1;
                }
                Value::MultiPoint(mp) | Value::LineString(mp) => {
                    for (coord_index, coord) in mp.iter().enumerate() {
                        if cb(coord as Position, coord_index, feature_index, multi_feature_index, geometry_index) == false {
                            return;
                        }
                    }

                }

                Value::MultiLineString(ｆ) | Value::Polygon(ｆ) => {}

                Value::MultiPolygon(_) => {}
                Value::GeometryCollection(_) => {}
            }
        }
    }
}

pub fn coord_each(geojson: GeoJson, cb: Callback, exclude_wrap_coord: bool) -> () {
    match geojson {
        GeoJson::Geometry(g) => {
            // e(vec![g])
        }
        GeoJson::Feature(f) => {
            e(vec![f], cb, exclude_wrap_coord)
        }
        GeoJson::FeatureCollection(fs) => {
            e(fs.features, cb, exclude_wrap_coord)
        }
    }
}