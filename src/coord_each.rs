// use geojson:{F }

use geojson::{Feature, GeoJson, Geometry, Position, Value};


// currentCoord, coordIndex, featureIndex, multiFeatureIndex, geometryIndex


// pub type Callback = Fn(&Position, usize, usize, usize, usize) -> bool;

fn get_wrap_shrink(exclude_wrap_coord: bool, gemo: &Value) -> usize {
    if !exclude_wrap_coord {
        return 0;
    }
    match gemo {
        Value::Polygon(_) | Value::MultiPolygon(_) => 1,
        _ => 0
    }
}

fn e<F>(features: &Vec<Feature>, mut cb: F, exclude_wrap_coord: bool)
    where
        F: FnMut(&Position, usize, usize, usize, usize) -> bool
{
    let mut coord_index: usize = 0;
    let mut multi_feature_index: usize = 0;
    let mut geometry_index: usize = 0;

    for (feature_index, feature) in features.iter().enumerate()
    {
        match &feature.geometry {
            None => {
                continue;
            }
            Some(g) => {
                let geom_type = &g.value;
                let wrap_shrink = get_wrap_shrink(exclude_wrap_coord, &geom_type);

                match geom_type {
                    Value::Point(p) => {
                        if cb(&p, coord_index, feature_index, multi_feature_index, geometry_index) == false {
                            return;
                        }
                        coord_index += 1;
                        multi_feature_index += 1;
                    }
                    Value::MultiPoint(mp) | Value::LineString(mp) => {
                        for coord in mp {
                            if cb(&coord, coord_index, feature_index, multi_feature_index, geometry_index) == false {
                                return;
                            }
                            if let Value::MultiPoint(_) = geom_type {
                                multi_feature_index += 1;
                            }
                            coord_index += 1;
                        }
                        if let Value::LineString(_) = geom_type {
                            multi_feature_index += 1;
                        }
                    }

                    Value::MultiLineString(ms) | Value::Polygon(ms) => {
                        for coord in ms {
                            let mut k = 0usize;
                            loop {
                                // if coord.get(k).is_none() {
                                //     k += 1;
                                //     continue;
                                // }


                                if cb(&coord[1], coord_index, feature_index, multi_feature_index, geometry_index) == false {
                                    return;
                                }
                                coord_index += 1;

                                k += 1;
                                if k >= (coord.len() - wrap_shrink) {
                                    break;
                                }
                            }
                            if let Value::MultiLineString(_) = geom_type {
                                multi_feature_index += 1;
                            }
                            if let Value::Polygon(_) = geom_type {
                                geometry_index += 1;
                            }
                        }
                        if let Value::Polygon(_) = geom_type {
                            multi_feature_index += 1;
                        }
                    }

                    Value::MultiPolygon(mp) => {
                        for polygon in mp {
                            geometry_index = 0;
                            let mut k = 0usize;

                            for linestring in polygon {
                                loop {
                                    if cb(&linestring[k], coord_index, feature_index, multi_feature_index, geometry_index) == false {
                                        return;
                                    }
                                    coord_index += 1;

                                    k += 1;
                                    if k >= (linestring.len() - wrap_shrink) {
                                        break;
                                    }
                                }
                                geometry_index += 1;
                            }
                            multi_feature_index += 1;
                        }
                    }
                    Value::GeometryCollection(_) => {}
                }
            }
        }
    }
}

pub fn coord_each<F>(geojson: &GeoJson, mut cb: F, exclude_wrap_coord: bool)
    where
        F: FnMut(&Position, usize, usize, usize, usize) -> bool
{
    match geojson {
        GeoJson::Geometry(_) => {
            // e(vec![g])
        }
        GeoJson::Feature(f) => {
            let sss = &vec![f.clone()];
            e(sss, cb, exclude_wrap_coord)
        }
        GeoJson::FeatureCollection(fs) => {
            e(&fs.features, cb, exclude_wrap_coord)
        }
    }
}


#[cfg(test)]
pub(crate) mod tests {
    use geojson::{FeatureCollection, JsonObject, JsonValue};
    use super::*;

    fn pt() -> Feature {
        let mut properties = JsonObject::new();
        properties.insert("1".to_string(), JsonValue::from("Firestone Grill"));

        Feature {
            bbox: None,
            properties: Some(properties),
            geometry: Some(Geometry::new(Value::Point(vec![1.000, 2.000]))),
            id: None,
            foreign_members: None,
        }
    }

    fn collection(f: &Feature) -> FeatureCollection {
        FeatureCollection {
            bbox: None,
            features: vec![f.clone()],
            foreign_members: None,
        }
    }

    #[test]
    fn coord_each__point() {
        // let int_array = [1, 2, 3, 4];
        // int_array.map(|i| {
        //     i + 1
        // });
        //
        // println!("{}", 222);
        let input = GeoJson::FeatureCollection(collection(&pt()));
        coord_each(&input, |coord: &Position, index: usize, v: usize, b: usize, x: usize| {
            assert_eq!(coord, &vec![1.000, 2.000]);
            assert_eq!(index, 0);
            return false;
        }, false)
    }
}