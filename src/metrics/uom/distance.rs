use crate::metrics::uom::UnitOfMeasureValueKind;
use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;

const METER_PER_FOOT: f64 = 0.3048;
const METER_PER_MILE: f64 = 1609.34;
const METER_PER_INCH: f64 = 0.0254;
const METER_PER_KILOMETER: f64 = 1000.0;
pub const EQUATORIAL_RADIUS_OF_EARTH: DistanceKind = DistanceKind::Kilometers(6378.0);
pub const POLAR_RADIUS_OF_EARTH: DistanceKind = DistanceKind::Kilometers(6357.0);

#[derive(Debug)]
pub enum DistanceKind {
    Feet(f64),
    Miles(f64),
    Meters(f64),
    Kilometers(f64),
    Inches(f64),
    Unknown,
}

impl fmt::Display for DistanceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            DistanceKind::Feet(ft) => { write!(f, "( Feet: {} )", ft) }
            DistanceKind::Miles(mi) => { write!(f, "( Miles: {} )", mi) }
            DistanceKind::Kilometers(km) => { write!(f, "( Kilometers: {} )", km) }
            DistanceKind::Meters(m) => { write!(f, "( Meters: {} )", m) }
            DistanceKind::Inches(i) => { write!(f, "( Inches: {} )", i) }
            DistanceKind::Unknown => { write!(f, "( Unknown") }
        }
    }
}

impl DistanceKind {
    pub fn to_feet(&self) -> Self {
        let meter: DistanceKind = self.to_si();
        let mut i: f64 = 0.0;
        if let DistanceKind::Meters(m) = meter { i = m / METER_PER_FOOT };
        DistanceKind::Feet(i)
    }
    pub fn to_miles(&self) -> Self {
        let meter: DistanceKind = self.to_si();
        let mut i: f64 = 0.0;
        if let DistanceKind::Meters(m) = meter { i = m / METER_PER_MILE };
        DistanceKind::Miles(i)
    }
    pub fn to_inches(&self) -> Self {
        let meter: DistanceKind = self.to_si();
        let mut i: f64 = 0.0;
        if let DistanceKind::Meters(m) = meter { i = m / METER_PER_INCH };
        DistanceKind::Inches(i)
    }
    pub fn to_kilometers(&self) -> Self {
        let meter: DistanceKind = self.to_si();
        let mut i: f64 = 0.0;
        if let DistanceKind::Meters(m) = meter { i = m / METER_PER_KILOMETER };
        DistanceKind::Kilometers(i)
    }
}

impl Sub<DistanceKind> for DistanceKind {
    type Output = DistanceKind;

    fn sub(self, rhs: DistanceKind) -> <Self as Sub<DistanceKind>>::Output{
        let l = self.to_si().get_value().unwrap();
        let r = rhs.to_si().get_value().unwrap();
        DistanceKind::Meters(l - r)
    }
}

impl Add<DistanceKind> for DistanceKind {
    type Output = DistanceKind;

    fn add(self, rhs: DistanceKind) -> <Self as Add<DistanceKind>>::Output {
        let l = self.to_si().get_value().unwrap();
        let r = rhs.to_si().get_value().unwrap();
        DistanceKind::Meters(l + r)
    }
}

impl AddAssign for DistanceKind {
    fn add_assign(&mut self, rhs: DistanceKind) {
        let l = self.to_si().get_value().unwrap();
        let r = rhs.to_si().get_value().unwrap();
        *self = DistanceKind::Meters(l + r);
    }
}

impl UnitOfMeasureValueKind for DistanceKind {
    fn get_value(&self) -> Option<f64> {
        match &self {
            DistanceKind::Feet(x) => { Some(*x) }
            DistanceKind::Miles(x) => { Some(*x) }
            DistanceKind::Meters(x) => { Some(*x) }
            DistanceKind::Kilometers(x) => { Some(*x) }
            DistanceKind::Inches(x) => { Some(*x) }
            DistanceKind::Unknown => None,
        }
    }

    fn set_value(&mut self, value: f64) -> &Self {
        match &self {
            DistanceKind::Feet(_) => { *self = DistanceKind::Feet(value); }
            DistanceKind::Miles(_) => { *self = DistanceKind::Miles(value); }
            DistanceKind::Meters(_) => { *self = DistanceKind::Meters(value); }
            DistanceKind::Kilometers(_) => { *self = DistanceKind::Kilometers(value); }
            DistanceKind::Inches(_) => { *self = DistanceKind::Inches(value); }
            DistanceKind::Unknown => { *self = DistanceKind::Unknown; }
        }
        self
    }

    fn to_si(&self) -> Self {
        match self {
            DistanceKind::Feet(ft) => DistanceKind::Meters(ft * METER_PER_FOOT),
            DistanceKind::Miles(mi) => DistanceKind::Meters(mi * METER_PER_MILE),
            DistanceKind::Kilometers(m) => DistanceKind::Meters(m * METER_PER_KILOMETER),
            DistanceKind::Meters(m) => DistanceKind::Meters(*m),
            DistanceKind::Inches(i) => DistanceKind::Meters(i * METER_PER_INCH),
            _ => DistanceKind::Unknown,
        }
    }
}

