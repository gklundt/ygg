use crate::metrics::uom::*;

pub fn distance_between_two_points(x: &PositionKind, y: &PositionKind) -> DistanceKind {
    match (x, y) {
        (PositionKind::TwoDimensionEuclidean { x: _, y: _ }, PositionKind::TwoDimensionEuclidean { x: _, y: _ }) => distance_between_two_points_(x, y, distance_between_two_points_2de),
        (PositionKind::ThreeDimensionEuclidean { x: _, y: _, z: _ }, PositionKind::ThreeDimensionEuclidean { x: _, y: _, z: _ }) => distance_between_two_points_(x, y, distance_between_two_points_3de),
        _ => DistanceKind::Unknown,
    }
}

fn distance_between_two_points_(x: &PositionKind, y: &PositionKind, f: fn(&PositionKind, &PositionKind) -> DistanceKind) -> DistanceKind {
    f(x, y)
}

fn distance_between_two_points_3de(x: &PositionKind, y: &PositionKind) -> DistanceKind {
    let x: PositionKind = x.to_si();
    let y: PositionKind = y.to_si();
    let mut x1: f64 = 0.0;
    let mut x2: f64 = 0.0;
    let mut y1: f64 = 0.0;
    let mut y2: f64 = 0.0;
    let mut z1: f64 = 0.0;
    let mut z2: f64 = 0.0;
    if let PositionKind::ThreeDimensionEuclidean { x, y, z } = x {
        if let DistanceKind::Meters(x) = x { x1 = x; }
        if let DistanceKind::Meters(y) = y { y1 = y; }
        if let DistanceKind::Meters(z) = z { z1 = z; }
    };
    if let PositionKind::ThreeDimensionEuclidean { x, y, z } = y {
        if let DistanceKind::Meters(x) = x { x2 = x; }
        if let DistanceKind::Meters(y) = y { y2 = y; }
        if let DistanceKind::Meters(z) = z { z2 = z; }
    };
    DistanceKind::Meters(((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0) + (z2 - z1).powf(2.0)).sqrt())
}

fn distance_between_two_points_2de(x: &PositionKind, y: &PositionKind) -> DistanceKind {
    let x: PositionKind = x.to_si();
    let y: PositionKind = y.to_si();
    let mut x1: f64 = 0.0;
    let mut x2: f64 = 0.0;
    let mut y1: f64 = 0.0;
    let mut y2: f64 = 0.0;
    if let PositionKind::TwoDimensionEuclidean { x, y } = x {
        if let DistanceKind::Meters(x) = x { x1 = x; }
        if let DistanceKind::Meters(y) = y { y1 = y; }
    };
    if let PositionKind::TwoDimensionEuclidean { x, y } = y {
        if let DistanceKind::Meters(x) = x { x2 = x; }
        if let DistanceKind::Meters(y) = y { y2 = y; }
    };
    DistanceKind::Meters(((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt())
}