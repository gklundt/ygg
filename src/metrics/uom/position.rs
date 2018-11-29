use crate::metrics::uom::distance;
use crate::metrics::uom::angles;
use crate::metrics::uom::UnitOfMeasureValueKind;
use std::fmt;

#[derive(Debug)]
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
        unimplemented!()
    }

    fn to_si(&self) -> Self {
        match self {
            PositionKind::TwoDimensionEuclidean { x, y } => PositionKind::TwoDimensionEuclidean { x: x.to_si(), y: y.to_si() },
            PositionKind::ThreeDimensionEuclidean { x, y, z } => PositionKind::ThreeDimensionEuclidean { x: x.to_si(), y: y.to_si(), z: z.to_si() },
            PositionKind::TwoDimensionGeo { lat, lng } => PositionKind::TwoDimensionGeo { lat: lat.to_si(), lng: lng.to_si() },
            PositionKind::Polar { radial, theta } => PositionKind::Polar { radial: radial.to_si(), theta: theta.to_si() },
            PositionKind::Spherical { radial, polar, azimuth } => PositionKind::Spherical { radial: radial.to_si(), polar: polar.to_si(), azimuth: azimuth.to_si() },
            PositionKind::Unknown => PositionKind::Unknown,
        }
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
