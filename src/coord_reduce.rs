use geojson::{GeoJson, Position};
use crate::coord_each::{coord_each};

pub fn coord_reduce<T, F>(geojson: &GeoJson, mut cb: F, initial_value: T, exclude_wrap_coord: bool) -> T
    where
        F: FnMut(&T, &Position, usize, usize, usize, usize) -> T
{
    let mut previous_value = initial_value;
    coord_each(
        geojson,
        |coord: &geojson::Position, coord_index: usize, v: usize, b: usize, x: usize| {
            previous_value = cb(&mut previous_value, coord, coord_index, v, b, x);
            return true;
        },
        exclude_wrap_coord);
    previous_value
}


#[cfg(test)]
pub(crate) mod tests {
    use geojson::{FeatureCollection, JsonObject, Geometry, JsonValue, Value};
    use super::*;

    #[test]
    fn coord_reduce__initial_value() {
        let input = GeoJson::Feature(
            geojson::Feature {
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
                properties: None,
                foreign_members: None,
            }
        );

        let mut last_index = 0;
        let sum = coord_reduce(
            &input,
            |previos: &f64, coord: &Position, index: usize, v: usize, b: usize, x: usize| {
                last_index = index;
                return previos + coord[0];
            },
            0.00,
            false,
        );

        assert_eq!(last_index, 2);
        assert_eq!(sum, 390.00);

    }
}