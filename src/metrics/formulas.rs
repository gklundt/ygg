use crate::metrics::uom::position;
use crate::metrics::uom::distance::DistanceKind;
use crate::metrics::uom::UnitOfMeasureValueKind;
use core::borrow::Borrow;


pub fn distance_between_two_points(x: Box<position::PositionKind>, y: Box<position::PositionKind>) -> DistanceKind {
    let x_si = x.clone().as_standard_unit().clone();
    let y_si = y.clone().as_standard_unit().clone();

    match (&x_si, &y_si) {
        (position::PositionKind::TwoDimensionEuclidean { x: _, y: _ },
            position::PositionKind::TwoDimensionEuclidean { x: _, y: _ })
        => distance_between_two_points_(Box::new(x_si), Box::new(y_si), distance_between_two_points_2de),

        (position::PositionKind::ThreeDimensionEuclidean { x: _, y: _, z: _ },
            position::PositionKind::ThreeDimensionEuclidean { x: _, y: _, z: _ })
        => distance_between_two_points_(Box::new(x_si), Box::new(y_si), distance_between_two_points_3de),

        _ => DistanceKind::Unknown,
    }
}

fn distance_between_two_points_(
    x: Box<position::PositionKind>,
    y: Box<position::PositionKind>,
    f: fn(Box<position::PositionKind>, Box<position::PositionKind>) -> DistanceKind) -> DistanceKind {
    f(x, y)
}

fn distance_between_two_points_3de(x: Box<position::PositionKind>, y: Box<position::PositionKind>) -> DistanceKind {

    let a = x.borrow();
    let b = y.borrow();

    let mut x1: f64 = 0.0;
    let mut x2: f64 = 0.0;
    let mut y1: f64 = 0.0;
    let mut y2: f64 = 0.0;
    let mut z1: f64 = 0.0;
    let mut z2: f64 = 0.0;
    if let position::PositionKind::ThreeDimensionEuclidean { x, y, z } = a {
        if let DistanceKind::Meters(x) = x { x1 = *x; }
        if let DistanceKind::Meters(y) = y { y1 = *y; }
        if let DistanceKind::Meters(z) = z { z1 = *z; }
    };
    if let position::PositionKind::ThreeDimensionEuclidean { x, y, z } = b {
        if let DistanceKind::Meters(x) = x { x2 = *x; }
        if let DistanceKind::Meters(y) = y { y2 = *y; }
        if let DistanceKind::Meters(z) = z { z2 = *z; }
    };
    DistanceKind::Meters(((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0) + (z2 - z1).powf(2.0)).sqrt())
}

fn distance_between_two_points_2de(x: Box<position::PositionKind>, y: Box<position::PositionKind>) -> DistanceKind {
    let a = x.borrow();
    let b = y.borrow();

    let mut x1: f64 = 0.0;
    let mut x2: f64 = 0.0;
    let mut y1: f64 = 0.0;
    let mut y2: f64 = 0.0;
    if let position::PositionKind::TwoDimensionEuclidean { x, y } = a {
        if let DistanceKind::Meters(x) = x { x1 = *x; }
        if let DistanceKind::Meters(y) = y { y1 = *y; }
    };
    if let position::PositionKind::TwoDimensionEuclidean { x, y } = b {
        if let DistanceKind::Meters(x) = x { x2 = *x; }
        if let DistanceKind::Meters(y) = y { y2 = *y; }
    };
    DistanceKind::Meters(((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt())
}