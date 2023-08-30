use std::fmt;
use geojson::{GeoJson, Position, FeatureCollection, PointType, Feature, JsonObject, Value, LineStringType};


pub fn prop_each<F>(geojson: &GeoJson, mut cb: F)
    where
        F: FnMut(&Option<JsonObject>, usize) -> bool
{
    match geojson {
        GeoJson::Geometry(_) => {}
        GeoJson::Feature(f) => {
            let _ = cb(&f.properties, 0);
        }
        GeoJson::FeatureCollection(collection) => {
            for (index, feature) in collection.features.iter().enumerate() {
                if cb(&feature.properties, index) == false {
                    break;
                }
            }
        }
    }
}


#[cfg(test)]
pub(crate) mod tests {
    use geojson::{FeatureCollection, JsonObject, Geometry, JsonValue, Value};
    use super::*;

    #[test]
    fn prop_each_test() {
        let mut properties = JsonObject::new();
        properties.insert("key".to_string(), JsonValue::from("Firestone Grill"));
        let input = GeoJson::FeatureCollection(
            FeatureCollection {
                bbox: None,
                features: vec![geojson::Feature {
                    bbox: None,
                    geometry: Some(
                        Geometry::new(
                            Value::LineString(vec![
                                vec![126.00, -11.00],
                                vec![129.00, -21.00],
                                vec![135.00, -31.00],
                            ])
                        )),
                    id: None,
                    properties: Some(properties),
                    foreign_members: None,
                }],
                foreign_members: None,
            }
        );

        prop_each(&input, |props, index| {
            assert_eq!(index, 0);
            if let Some(obj)= props {
                if let Some(key) = obj.get("key") {
                    assert_eq!(key, "Firestone Grill")
                }
            }
            return true;
        });

    }
}