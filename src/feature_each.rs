use geojson::{GeoJson, Position, FeatureCollection, PointType, Feature, JsonObject, Value, LineStringType};


pub fn feature_each<F>(geojson: &GeoJson, mut cb: F)
    where
        F: FnMut(&Feature, usize) -> bool
{
    match geojson {
        GeoJson::Geometry(_) => {}
        GeoJson::Feature(f) => {
            let _ = cb(f, 0);
        }
        GeoJson::FeatureCollection(collection) => {
            for (index, feature) in collection.features.iter().enumerate() {
                if cb(&feature, index) == false {
                    break;
                }
            }
        }
    }
}

pub fn feature_reduce<T, F>(geojson: &GeoJson, mut cb: F, initial_value: T) -> T
    where
        F: FnMut(&T, &Feature, usize) -> T
{
    let mut previous_value = initial_value;
    feature_each(
        geojson,
        |f, index| {
            previous_value = cb(&previous_value, f, index);
            return true;
        });
    previous_value
}


#[cfg(test)]
pub(crate) mod tests {
    use geojson::{FeatureCollection, JsonObject, Geometry, JsonValue, Value};
    use super::*;
}