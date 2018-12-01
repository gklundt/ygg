use crate::metrics::uom::distance;
use crate::metrics::uom::angles;
use crate::metrics::uom::UnitOfMeasureValueKind;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum PositionKind {
    TwoDimensionEuclidean { x: distance::DistanceKind, y: distance::DistanceKind },
    ThreeDimensionEuclidean { x: distance::DistanceKind, y: distance::DistanceKind, z: distance::DistanceKind },
    TwoDimensionGeo { lat: angles::AngularKind, lng: angles::AngularKind },
    Spherical { radial: distance::DistanceKind, polar: angles::AngularKind, azimuth: angles::AngularKind },
    Polar { radial: distance::DistanceKind, theta: angles::AngularKind },
    Unknown,
}

impl UnitOfMeasureValueKind for PositionKind {
    fn get_value(&self) -> Option<f64> {
        unimplemented!()
    }

    fn set_value(&mut self, value: f64) -> &Self {
        let _ = value;
        unimplemented!()
    }

    fn as_standard_unit(&mut self) -> &Self {
        match self {
            PositionKind::TwoDimensionEuclidean { x, y } =>
                { *self = PositionKind::TwoDimensionEuclidean { x: x.clone().as_standard_unit().clone(), y: y.clone().as_standard_unit().clone() } }
            PositionKind::ThreeDimensionEuclidean { x, y, z } =>
                { *self = PositionKind::ThreeDimensionEuclidean { x: x.clone().as_standard_unit().clone(), y: y.clone().as_standard_unit().clone(), z: z.clone().as_standard_unit().clone() } }
            PositionKind::TwoDimensionGeo { lat, lng } =>
                { *self = PositionKind::TwoDimensionGeo { lat: lat.clone().as_standard_unit().clone(), lng: lng.clone().as_standard_unit().clone() } }
            PositionKind::Polar { radial, theta } =>
                { *self = PositionKind::Polar { radial: radial.clone().as_standard_unit().clone(), theta: theta.clone().as_standard_unit().clone() } }
            PositionKind::Spherical { radial, polar, azimuth } =>
                { *self = PositionKind::Spherical { radial: radial.clone().as_standard_unit().clone(), polar: polar.clone().as_standard_unit().clone(), azimuth: azimuth.clone().as_standard_unit().clone() } }
            PositionKind::Unknown =>
                { *self = PositionKind::Unknown }
        }
        self
    }
}

impl fmt::Display for PositionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            PositionKind::TwoDimensionEuclidean { x, y } => { write!(f, "( x: {}, y: {} )", x, y) }
            PositionKind::ThreeDimensionEuclidean { x, y, z } => { write!(f, "( x: {}, y: {}, z: {} )", x, y, z) }
            PositionKind::TwoDimensionGeo { lat, lng } => { write!(f, "( lat: {}, long: {} )", lat, lng) }
            PositionKind::Polar { radial, theta } => { write!(f, "( radius: {}, theta: {} )", radial, theta) }
            PositionKind::Spherical { radial, polar, azimuth } => { write!(f, "( radial: {}, polar: {}, azimuth: {} )", radial, polar, azimuth) }
            PositionKind::Unknown => { write!(f, "( Unknown Position )") }
        }
    }
}
