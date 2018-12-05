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

#[derive(Debug, Clone)]
pub enum DistanceKind {
    Feet(f64),
    Miles(f64),
    Meters(f64),
    Kilometers(f64),
    Inches(f64),
    Unknown,
}

impl DistanceKind {
    pub fn as_feet(&mut self) -> &Self {
        self.as_standard_unit();
        if let Some(m) = self.get_value() { *self = DistanceKind::Feet(m / METER_PER_FOOT) } else {
            *self = DistanceKind::Unknown
        }
        self
    }
    pub fn as_miles(&mut self) -> &Self {
        self.as_standard_unit();
        if let Some(m) = self.get_value() { *self = DistanceKind::Miles(m / METER_PER_MILE) } else {
            *self = DistanceKind::Unknown
        }
        self
    }
    pub fn as_inches(&mut self) -> &Self {
        self.as_standard_unit();
        if let Some(m) = self.get_value() { *self = DistanceKind::Inches(m / METER_PER_INCH) } else {
            *self = DistanceKind::Unknown
        }
        self
    }
    pub fn as_kilometers(&mut self) -> &Self {
        self.as_standard_unit();

        if let Some(m) = self.get_value() { *self = DistanceKind::Kilometers(m / METER_PER_KILOMETER) } else {
            *self = DistanceKind::Unknown
        }
        self
    }
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

impl Sub<DistanceKind> for DistanceKind {
    type Output = DistanceKind;

    fn sub(self, rhs: DistanceKind) -> Self {
        let l_value = self.clone().as_standard_unit().get_value().unwrap();
        let r_value = rhs.clone().as_standard_unit().get_value().unwrap();
        DistanceKind::Meters(l_value - r_value)
    }
}

impl Add<DistanceKind> for DistanceKind {
    type Output = DistanceKind;

    fn add(self, rhs: DistanceKind) -> Self {
        let l_value = self.clone().as_standard_unit().get_value().unwrap();
        let r_value = rhs.clone().as_standard_unit().get_value().unwrap();
        DistanceKind::Meters(l_value + r_value)
    }
}

impl AddAssign for DistanceKind {
    fn add_assign(&mut self, rhs: DistanceKind) {
        let l_value = self.as_standard_unit().get_value().unwrap();
        let r_value = rhs.clone().as_standard_unit().get_value().unwrap();
        self.set_value(l_value + r_value);
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

    fn as_standard_unit(&mut self) -> &mut Self {
        match self {
            DistanceKind::Feet(ft) => { *self = DistanceKind::Meters(*ft * METER_PER_FOOT) }
            DistanceKind::Miles(mi) => { *self = DistanceKind::Meters(*mi * METER_PER_MILE) }
            DistanceKind::Kilometers(m) => { *self = DistanceKind::Meters(*m * METER_PER_KILOMETER) }
            DistanceKind::Meters(m) => { *self = DistanceKind::Meters(*m) }
            DistanceKind::Inches(i) => { *self = DistanceKind::Meters(*i * METER_PER_INCH) }
            _ => { *self = DistanceKind::Unknown }
        }
        self
    }
}

