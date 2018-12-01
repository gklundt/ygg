use crate::metrics::uom::position;
use crate::metrics::uom::distance::DistanceKind;
use crate::metrics::uom::UnitOfMeasureValueKind;

pub fn distance_between_two_points(x: position::PositionKind, y: position::PositionKind) -> DistanceKind {
    match (x, y) {
        (position::PositionKind::TwoDimensionEuclidean { x: _, y: _ },
            position::PositionKind::TwoDimensionEuclidean { x: _, y: _ })
        => distance_between_two_points_(x.clone(), y.clone(), distance_between_two_points_2de),

        (position::PositionKind::ThreeDimensionEuclidean { x: _, y: _, z: _ },
            position::PositionKind::ThreeDimensionEuclidean { x: _, y: _, z: _ })
        => distance_between_two_points_(x.clone(), y.clone(), distance_between_two_points_3de),

        _ => DistanceKind::Unknown,
    }
}

fn distance_between_two_points_(x: position::PositionKind, y: position::PositionKind, f: fn(position::PositionKind, position::PositionKind) -> DistanceKind) -> DistanceKind {
    f(x, y)
}

fn distance_between_two_points_3de(mut x: position::PositionKind, mut y: position::PositionKind) -> DistanceKind {
    x.as_standard_unit();
    y.as_standard_unit();
    let mut x1: f64 = 0.0;
    let mut x2: f64 = 0.0;
    let mut y1: f64 = 0.0;
    let mut y2: f64 = 0.0;
    let mut z1: f64 = 0.0;
    let mut z2: f64 = 0.0;
    if let position::PositionKind::ThreeDimensionEuclidean { x, y, z } = x {
        if let DistanceKind::Meters(x) = x { x1 = x; }
        if let DistanceKind::Meters(y) = y { y1 = y; }
        if let DistanceKind::Meters(z) = z { z1 = z; }
    };
    if let position::PositionKind::ThreeDimensionEuclidean { x, y, z } = y {
        if let DistanceKind::Meters(x) = x { x2 = x; }
        if let DistanceKind::Meters(y) = y { y2 = y; }
        if let DistanceKind::Meters(z) = z { z2 = z; }
    };
    DistanceKind::Meters(((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0) + (z2 - z1).powf(2.0)).sqrt())
}

fn distance_between_two_points_2de(mut x: position::PositionKind, mut y: position::PositionKind) -> DistanceKind {
    x.as_standard_unit();
    y.as_standard_unit();
    let mut x1: f64 = 0.0;
    let mut x2: f64 = 0.0;
    let mut y1: f64 = 0.0;
    let mut y2: f64 = 0.0;
    if let position::PositionKind::TwoDimensionEuclidean { x, y } = x {
        if let DistanceKind::Meters(x) = x { x1 = x; }
        if let DistanceKind::Meters(y) = y { y1 = y; }
    };
    if let position::PositionKind::TwoDimensionEuclidean { x, y } = y {
        if let DistanceKind::Meters(x) = x { x2 = x; }
        if let DistanceKind::Meters(y) = y { y2 = y; }
    };
    DistanceKind::Meters(((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt())
}