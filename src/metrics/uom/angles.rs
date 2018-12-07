use std::fmt;
use crate::metrics::uom::UnitOfMeasureValueKind;
use std::f64::consts;


const RAD_PER_DEG: f64 = consts::PI / 180.0;

#[derive(Debug)]
pub enum AngularKind {
    Degrees(f64),
    Radians(f64),
    Unknown,
}

impl fmt::Display for AngularKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AngularKind::Degrees(deg) => { write!(f, "( Degrees: {} )", deg) }
            AngularKind::Radians(rad) => { write!(f, "( Radians: {} )", rad) }
            AngularKind::Unknown => { write!(f, "( Radians: Unknown )") }
        }
    }
}

impl AngularKind {
    pub fn as_deg(&mut self) -> &Self {
        self.as_standard_unit();
        if let Some(r) = self.get_value() { *self = AngularKind::Degrees(r / RAD_PER_DEG); }
        self
    }

    pub fn as_deg_mod(&mut self) -> &Self {
        self.as_standard_unit();
        if let Some(r) = self.get_value() { *self = AngularKind::Degrees((r / RAD_PER_DEG) % 360.0); }
        self
    }
}

impl UnitOfMeasureValueKind for AngularKind {
    fn get_value(&self) -> Option<f64> {
        match &self {
            AngularKind::Degrees(d) => Some(*d),
            AngularKind::Radians(r) => Some(*r),
            AngularKind::Unknown => None,
        }
    }

    fn set_value(&mut self, value: f64) -> &Self {
        match &self {
            AngularKind::Degrees(_) => { *self = AngularKind::Degrees(value); }
            AngularKind::Radians(_) => { *self = AngularKind::Radians(value); }
            AngularKind::Unknown => { *self = AngularKind::Unknown; }
        }
        self
    }

    fn as_standard_unit(&mut self) -> &mut Self {
        match self {
            AngularKind::Radians(r) => { *self = AngularKind::Radians(*r) }
            AngularKind::Degrees(d) => { *self = AngularKind::Radians(*d * RAD_PER_DEG) }
            AngularKind::Unknown => { *self = AngularKind::Unknown }
        }
        self
    }

    fn duplicate(&self) -> Self where Self: Sized {
        match self {
            AngularKind::Radians(r) => AngularKind::Radians(*r),
            AngularKind::Degrees(d) => AngularKind::Degrees(*d),
            AngularKind::Unknown => AngularKind::Unknown,
        }
    }
}
