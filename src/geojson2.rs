// type Point = Vec<f64>;
//
// trait Feature {
//     fn new(coord: Self::Coord) -> Self;
// }
//
// struct FPoint {
//     coord: Point
// }
//
// impl Feature for FPoint {
//     type Coord = Point;
//
//     fn new(coord: Self::Coord) -> Self {
//         todo!()
//     }
// }
//
// struct FLineString {
//     coord: Vec<Point>
// }
//
// impl Feature for FLineString {
//     type Coord = Vec<Point>;
//
//     fn new(coord: Self::Coord) -> Self {
//         todo!()
//     }
// }
//
//
// struct Collection {
//     features: Vec<Feature>
// }
//
// enum geojson {
//     Feature(dyn Feature)
// }