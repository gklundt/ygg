use std::f64::consts;
use std::fmt;
use crate::metrics::uom::UnitOfMeasureValueKind;

const RAD_PER_DEG: f64 = consts::PI / 180.0;

#[derive(Debug)]
pub enum AngularKind {
    Degrees(f64),
    Radians(f64),
}

impl fmt::Display for AngularKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AngularKind::Degrees(deg) => { write!(f, "( Degrees: {} )", deg) }
            AngularKind::Radians(rad) => { write!(f, "( Radians: {} )", rad) }
        }
    }
}

impl AngularKind {
    pub fn to_deg(&self) -> Self {
        let si: AngularKind = self.to_si();
        let mut i: f64 = 0.0;
        if let AngularKind::Radians(r) = si { i = r / RAD_PER_DEG };
        AngularKind::Degrees(i)
    }

    pub fn to_deg_rem(&self) -> Self {
        let si: AngularKind = self.to_si();
        let mut i: f64 = 0.0;
        if let AngularKind::Radians(r) = si { i = (r / RAD_PER_DEG) % 360.0 };
        AngularKind::Degrees(i)
    }
}

impl UnitOfMeasureValueKind for AngularKind {
    fn get_value(&self) -> Option<f64> {
        unimplemented!()
    }

    fn set_value(&mut self, value: f64) -> &Self {
        unimplemented!()
    }

    fn to_si(&self) -> Self {
        match self {
            AngularKind::Radians(r) => AngularKind::Radians(*r),
            AngularKind::Degrees(d) => AngularKind::Radians(d * RAD_PER_DEG),
        }
    }
}
