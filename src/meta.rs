use geojson::{Bbox, feature, Feature, GeoJson, Geometry, JsonObject, Value};

// pub fn coord_each<F>(geojson: &GeoJson, mut cb: F, exclude_wrap_coord: bool)
//     where
//         F: FnMut(&Position, usize, usize, usize, usize) -> bool
// {
pub fn gemo_each<F>(geojson: &GeoJson, mut cb: F)
    where F: FnMut(Option<&Value>, usize, Option<&JsonObject>, Option<&Bbox>, Option<&feature::Id>) {
    let mut feature_index = 0_usize;
    match geojson {
        GeoJson::Geometry(g) => {
            cb(Some(&g.value), feature_index, None, g.bbox.as_ref(), None)
        }
        GeoJson::Feature(f) => {
            do_feature(f, &mut cb, feature_index)
        }
        GeoJson::FeatureCollection(c) => {
            c.features.iter().enumerate().for_each(|(index, feature)| {
                do_feature(feature, &mut cb, index)
            })
        }
    }
}

fn do_feature<F>(f: &Feature, cb: &mut F, feature_index: usize)
    where F: FnMut(Option<&Value>, usize, Option<&JsonObject>, Option<&Bbox>, Option<&feature::Id>) {
    match &f.geometry {
        None => {
            cb(None, feature_index, f.properties.as_ref(), f.bbox.as_ref(), f.id.as_ref())
        }
        Some(g) => {
            if let Value::GeometryCollection(geometry_collection) = &g.value {
                geometry_collection.iter().for_each(|g| {
                    cb(Some(&g.value), feature_index, f.properties.as_ref(), f.bbox.as_ref(), f.id.as_ref())
                })
            } else {
                cb(Some(&g.value), feature_index, f.properties.as_ref(), f.bbox.as_ref(), f.id.as_ref())
            }
        }
    }
}

pub fn gemo_reduce<T, F>(geojson: &GeoJson, mut cb: F, initial_value: T) -> T
    where F: FnMut(&T, Option<&Value>, usize, Option<&JsonObject>, Option<&Bbox>, Option<&feature::Id>) -> T
{
    let mut previous_value = initial_value;
    gemo_each(
        geojson,
        |geom, index, props, bbix, id| {
            previous_value = cb(&mut previous_value, geom, index, props, bbix, id);
        });
    previous_value
}