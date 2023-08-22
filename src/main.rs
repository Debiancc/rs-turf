mod geojson;

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

    let linestring_feature = geojson::GeoJSONFeature {
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


}
