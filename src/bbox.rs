use std::f64::INFINITY;
use geojson::{Bbox, GeoJson};
use crate::coord_each::coord_each;

pub fn bbox(geojson: &GeoJson, recompute: bool) -> Bbox {
    if !recompute {
        match geojson {
            GeoJson::Geometry(g) => {
                if g.bbox.is_some() {
                    return g.bbox.clone().unwrap();
                }
            }
            GeoJson::Feature(f) => {
                if f.bbox.is_some() {
                    return f.bbox.clone().unwrap();
                }
            }
            GeoJson::FeatureCollection(fc) => {
                if fc.bbox.is_some() {
                    return fc.bbox.clone().unwrap();
                }
            }
        }
    }

    let mut result: Bbox = vec![f64::INFINITY, f64::INFINITY, -f64::INFINITY, -f64::INFINITY];
    coord_each(geojson, |coord, _, _, _, _| {
        if result[0] > coord[0] {
            result[0] = coord[0]
        }

        if result[1] > coord[1] {
            result[1] = coord[1]
        }

        if result[2] < coord[0] {
            result[2] = coord[0]
        }

        if result[3] < coord[1] {
            result[3] = coord[1]
        }
        true
    }, false);

    return result;
}


#[cfg(test)]
pub(crate) mod tests {
    use geojson::GeoJson::{Feature, FeatureCollection};
    use geojson::{GeoJson, Value};
    use crate::bbox::bbox;
    use crate::helper::new_feature;

    #[test]
    fn area_test() {
        let point = new_feature(Value::Point(vec![102., 0.5]));
        let line = new_feature(Value::LineString(vec![
            vec![102., -10.],
            vec![103., 1.],
            vec![104., 0.],
            vec![130., 4.],
        ]));
        let poly = new_feature(Value::Polygon(
            vec![
                vec![
                    vec![101., 0.],
                    vec![101., 1.],
                    vec![100., 1.],
                    vec![100., 0.],
                    vec![101., 0.],
                ]
            ]
        ));
        let multi_line = new_feature(Value::MultiLineString(vec![
            vec![
              vec![100.0, 0.0],
              vec![101.0, 1.0],
            ],
            vec![
                vec![102.0, 2.0],
                vec![103.0, 3.0],
            ],
        ]));
        let multi_poly = new_feature(Value::MultiPolygon(vec![
            vec![
                vec![
                    vec![102., 2.],
                    vec![103., 2.],
                    vec![103., 3.],
                    vec![102., 3.],
                    vec![102., 2.],
                ]
            ],
            vec![
                vec![
                    vec![100., 0.],
                    vec![101., 0.],
                    vec![101., 1.],
                    vec![100., 1.],
                    vec![100., 0.],
                ]
            ],
            vec![
                vec![
                    vec![100.2, 0.2],
                    vec![100.8, 0.2],
                    vec![100.8, 0.8],
                    vec![100.2, 0.8],
                    vec![100.2, 0.2],
                ]
            ],
        ]));

        assert_eq!(bbox(&GeoJson::Feature(point.clone()), false), vec![102., 0.5, 102., 0.5,]);
        assert_eq!(bbox(&GeoJson::Feature(line.clone()), false), vec![102., -10., 130., 4.,]);
        assert_eq!(bbox(&GeoJson::Feature(poly.clone()), false), vec![100., 0., 101., 1.,]);
        assert_eq!(bbox(&GeoJson::Feature(multi_line.clone()), false), vec![100., 0., 103., 3.,]);
        assert_eq!(bbox(&GeoJson::Feature(multi_poly.clone()), false), vec![100., 0., 103., 3.,]);

        let mut point_2 = point.clone();
        point_2.bbox = Some(vec![6.,3.,2.,4.]);
        assert_eq!(bbox(&GeoJson::Feature(point_2.clone()), false), vec![6.,3.,2.,4.], "default case");
        assert_eq!(bbox(&GeoJson::Feature(point_2), true), vec![102., 0.5, 102., 0.5,], "recompute case");

        let fc = FeatureCollection(geojson::FeatureCollection{
            bbox: None,
            features: vec![point, line, poly, multi_line, multi_poly],
            foreign_members: None,
        });

        assert_eq!(bbox(&fc, false), vec![100., -10., 130., 4.,]);





    }
}