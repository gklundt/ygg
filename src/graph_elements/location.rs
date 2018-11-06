use crate::metrics::uom;

#[derive(Debug)]
pub enum LocationKind {
    TwoDimensionEuclidean {
        x: uom::DistanceKind,
        y: uom::DistanceKind,
    },
    ThreeDimensionEuclidean {
        x: uom::DistanceKind,
        y: uom::DistanceKind,
        z: uom::DistanceKind,
    },
    TwoDimensionGeo {
        lat: uom::AngularKind,
        lng: uom::AngularKind,
    },
    Polar {
        radial: uom::DistanceKind,
        polar: uom::AngularKind,
        azimuth: uom::AngularKind,
    },
}
