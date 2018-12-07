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
        let _ = value;
        unimplemented!()
    }

    fn as_standard_unit(&mut self) -> &mut Self {
        match self {
            PositionKind::TwoDimensionEuclidean { x, y } =>
                {
                    *self = PositionKind::TwoDimensionEuclidean {
                        y: y.duplicate().as_standard_unit().duplicate(),
                        x: x.duplicate().as_standard_unit().duplicate(),
                    }
                }
            PositionKind::ThreeDimensionEuclidean { x, y, z } =>
                {
                    *self = PositionKind::ThreeDimensionEuclidean {
                        x: x.duplicate().as_standard_unit().duplicate(),
                        y: y.duplicate().as_standard_unit().duplicate(),
                        z: z.duplicate().as_standard_unit().duplicate(),
                    }
                }
            PositionKind::TwoDimensionGeo { lat, lng } =>
                {
                    *self = PositionKind::TwoDimensionGeo {
                        lat: lat.duplicate().as_standard_unit().duplicate(),
                        lng: lng.duplicate().as_standard_unit().duplicate(),
                    }
                }
            PositionKind::Polar { radial, theta } =>
                {
                    *self = PositionKind::Polar {
                        radial: radial.duplicate().as_standard_unit().duplicate(),
                        theta: theta.duplicate().as_standard_unit().duplicate(),
                    }
                }
            PositionKind::Spherical { radial, polar, azimuth } =>
                {
                    *self = PositionKind::Spherical {
                        radial: radial.duplicate().as_standard_unit().duplicate(),
                        polar: polar.duplicate().as_standard_unit().duplicate(),
                        azimuth: azimuth.duplicate().as_standard_unit().duplicate(),
                    }
                }
            PositionKind::Unknown =>
                { *self = PositionKind::Unknown }
        }
        self
    }

    fn duplicate(&self) -> Self where Self: Sized {
        match &self {
            PositionKind::TwoDimensionEuclidean { x, y } => PositionKind::TwoDimensionEuclidean { x: x.duplicate(), y: y.duplicate() },
            PositionKind::ThreeDimensionEuclidean { x, y, z } => PositionKind::ThreeDimensionEuclidean { x: x.duplicate(), y: y.duplicate(), z: z.duplicate() },
            PositionKind::TwoDimensionGeo { lat, lng } => PositionKind::TwoDimensionGeo { lat: lat.duplicate(), lng: lng.duplicate() },
            PositionKind::Polar { radial, theta } => PositionKind::Polar { radial: radial.duplicate(), theta: theta.duplicate() },
            PositionKind::Spherical { radial, polar, azimuth } => PositionKind::Spherical { radial: radial.duplicate(), polar: polar.duplicate(), azimuth: azimuth.duplicate() },
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
