use std::f64::consts::PI;
use geojson::{GeoJson, PolygonType, Position, Value};
use crate::meta::gemo_reduce;

pub fn area(geojson: &GeoJson) -> f64 {
    gemo_reduce(geojson, |value, gemo, index, props, bbox, id| {
        value + 1.
    }, 0.)
}

fn calc_area(geometry: &Value) -> f64 {
    match geometry {
        Value::Point(_) |
        Value::MultiPoint(_) |
        Value::LineString(_) |
        Value::MultiLineString(_) |
        Value::GeometryCollection(_) => { 0. }
        Value::Polygon(p) => { polygon_area(p) }
        Value::MultiPolygon(mp) => {
            let mut total = 0.;
            mp.iter().for_each(|polygon| {
                total += polygon_area(polygon);
            });
            total
        }
    }
}

fn polygon_area(polygon: &PolygonType) -> f64 {
    if polygon.is_empty() {
        return 0.;
    }

    let mut total = ring_area(&polygon[0]).abs();
    polygon.iter().for_each(|coords| {
        total -= ring_area(coords).abs()
    });
    total
}

fn ring_area(coords: &Vec<Position>) -> f64 {
    let mut total = 0.;
    let length = coords.len();
    if length > 2 {
        coords.iter().enumerate().for_each(|(index, coord)| {
            let mut p1: &Position;
            let mut p2: &Position;
            let mut p3: &Position;
            if index == length - 2 {
                p1 = &coords[length - 2];
                p2 = &coords[length - 1];
                p3 = &coords[0];
            } else if index == length - 1 {
                p1 = &coords[length - 1];
                p2 = &coords[0];
                p3 = &coords[1];
            } else {
                p1 = &coords[index];
                p2 = &coords[index + 1];
                p3 = &coords[index + 2];
            }

            total += (rad(p3[0]) - rad(p1[0])) * rad((p2[1])).sin()
        })
    }
    total
}

fn rad(num: f64) -> f64 {
    (num * PI) / 180.
}