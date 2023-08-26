extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(crate) enum GeoJSONGeometryType {
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
}

pub(crate) type LineString = Vec<Vec<f64>>;
pub(crate) type Point = Vec<f64>;


#[derive(Serialize, Deserialize)]
pub(crate) struct GeoJSONGeometry<T> {
    #[serde(rename = "type")]
    pub(crate) geometry_type: GeoJSONGeometryType,
    pub(crate) coordinates: T,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GeoJSONProperties {
    // Add your desired properties here
    pub(crate) name: String,
    pub(crate) category: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GeoJSONFeature<T> {
    #[serde(rename = "type")]
    pub(crate) feature_type: String,
    pub(crate) geometry: GeoJSONGeometry<T>,
    pub(crate) properties: GeoJSONProperties,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GeoJSONFeatureCollection<T> {
    #[serde(rename = "type")]
    pub(crate) collection_type: String,
    // pub(crate) features: Vec<(GeoJSONFeature<T>, GeoJSONFeature<T>)>
    pub(crate) features: Vec<Box<dyn GeoJSONFeature>>
}