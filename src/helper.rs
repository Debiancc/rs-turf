use std::f64::consts::PI;
use crate::types::Units;

pub const EARTH_RADIUS: f64 = 6371008.8;

impl Units {
    fn factor(&self) -> f64 {
        match self {
            Units::Meters | Units::Metres => { EARTH_RADIUS }
            Units::Millimeters | Units::Millimetres => { EARTH_RADIUS * 1000. }
            Units::Centimetres | Units::Centimeters => { EARTH_RADIUS * 100. }
            Units::Kilometres | Units::Kilometers => { EARTH_RADIUS / 1000. }
            Units::Miles => { EARTH_RADIUS / 1609.344 }
            Units::Nauticalmiles => { EARTH_RADIUS / 1852. }
            Units::Inches => { EARTH_RADIUS * 39.37 }
            Units::Yards => { EARTH_RADIUS * 1.0936 }
            Units::Feet => { EARTH_RADIUS * 3.28084 }
            Units::Radians => { 1. }
            Units::Degrees => { 360. / (2. * PI) }
        }
    }
}

trait OptionUnits {
    fn factor(&self) -> f64;
}

impl OptionUnits for Option<&Units> {
    fn factor(&self) -> f64 {
        match self {
            None => { Units::Kilometers.factor() }
            Some(u) => { u.factor() }
        }
    }
}


pub fn radians_to_degrees(radians: f64) -> f64 {
    let degrees = radians % (2. * PI);
    (degrees * 180.) / PI
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    let radians = degrees % 360.;
    (radians * PI) / 180.
}

pub fn length_to_radians(distance: f64, units: Option<&Units>) -> f64 {
    distance / units.factor()
}

pub fn radians_to_length(distance: f64, units: Option<&Units>) -> f64 {
    distance * units.factor()
}

pub fn bearing_to_azimuth(bearing: f64) -> f64 {
    let angle = bearing % 360.;
    if angle < 0. {
        return angle + 360.;
    }
    angle
}