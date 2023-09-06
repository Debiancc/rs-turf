use std::f64::consts::PI;
use geojson::{GeoJson, PolygonType, Position, Value};
use crate::helper::EARTH_RADIUS;
use crate::meta::gemo_reduce;

pub fn area(geojson: &GeoJson) -> f64 {
    gemo_reduce(geojson, |value, gemo, index, props, bbox, id| {
        if gemo.is_none() {
            return *value
        }
        value + calc_area(gemo.unwrap())
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
    println!("1 polygon_area = {}", total);
    polygon.iter().skip(1).for_each(|coords| {
        total -= ring_area(coords).abs()
    });

    println!("polygon_area = {}", total);
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
        });
        total = (total * EARTH_RADIUS * EARTH_RADIUS) / 2.;
    }
    total
}

fn rad(num: f64) -> f64 {
    (num * PI) / 180.
}


#[cfg(test)]
pub(crate) mod tests {
    use geojson::GeoJson::{Feature};
    use super::*;

    #[test]
    fn area_test() {
        let polygon = Value::Polygon(
            vec![
                vec![
                    vec![125., -15.],
                    vec![113., -22.],
                    vec![117., -37.],
                    vec![130., -33.],
                    vec![148., -39.],
                    vec![154., -27.],
                    vec![144., -15.],
                    vec![125., -15.],
                ]
            ]
        );
        let f = Feature(geojson::Feature{
            bbox: None,
            geometry: Some(geojson::Geometry::new(polygon)),
            id: None,
            properties: None,
            foreign_members: None,
        });

        assert_eq!(area(&f), 7748891609977.457);
    }
}