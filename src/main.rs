use crate::geojson::GeoJSONGeometryType::LineString;

mod geojson;
mod coord_each;

fn main() {
    println!("Hello, world!");

    // Create a LineString
    let linestring_geometry = geojson::GeoJSONGeometry {
        geometry_type: geojson::GeoJSONGeometryType::LineString,
        coordinates: vec![
            vec![100.0, 0.0],
            vec![101.0, 1.0],
            vec![102.0, 2.0],
        ],
    };

    let point_geometry = geojson::GeoJSONGeometry {
        geometry_type: geojson::GeoJSONGeometryType::Point,
        coordinates: vec![100.0, 0.0],
    };

    let linestring_feature:geojson::GeoJSONFeature<geojson::LineString> = geojson::GeoJSONFeature {
        feature_type: String::from("Feature"),
        geometry: linestring_geometry,
        properties: geojson::GeoJSONProperties {
            name: String::from("Sample LineString"),
            category: String::from("Test"),
        },
    };

    // Create a Polygon
    let polygon_geometry = geojson::GeoJSONGeometry {
        geometry_type: geojson::GeoJSONGeometryType::Polygon,
        coordinates: vec![
            vec![
                vec![100.0, 0.0],
                vec![101.0, 0.0],
                vec![101.0, 1.0],
                vec![100.0, 1.0],
                vec![100.0, 0.0],
            ],
        ],
    };

    let polygon_feature = geojson::GeoJSONFeature {
        feature_type: String::from("Feature"),
        geometry: polygon_geometry,
        properties: geojson::GeoJSONProperties {
            name: String::from("Sample Polygon"),
            category: String::from("Test"),
        },
    };


    let mixed_features: geojson::GeoJSONFeatureCollection<serde_json::Value> = geojson::GeoJSONFeatureCollection {
        collection_type: String::from("FeatureCollection"),
        features: vec![
            // linestring_feature.into(),
            // point_geometry,
            // point_geometry.clone(),
            //
            (linestring_feature.clone(), polygon_feature.clone()), // Tuple of GeoJSONFeature
            // (linestring_feature.clone(), polygon_feature.clone()), // Tuple of GeoJSONFeature
            // (linestring_feature.clone(), polygon_feature.clone()), // Tuple of GeoJSONFeature
            // (polygon_feature.clone(), linestring_feature.clone()), // Another tuple
            // (polygon_feature.clone(), linestring_feature.clone()), // Another tuple
            // (polygon_feature.clone(), linestring_feature.clone()), // Another tuple
            // (polygon_feature.clone(), linestring_feature.clone()), // Another tuple
            // (polygon_feature.clone(), linestring_feature.clone()), // Another tuple
        ],
    };

    println!("{}", mixed_features.features.len())


}
