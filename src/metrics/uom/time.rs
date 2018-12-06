use crate::metrics::uom::UnitOfMeasureValueKind;
use std::fmt;

#[derive(Debug)]
pub enum TimeKind {
    Hours(f64),
    Minutes(f64),
    Seconds(f64),
    Milliseconds(f64),
    Unknown,
}

impl UnitOfMeasureValueKind for TimeKind {
    fn get_value(&self) -> Option<f64> {
        match &self {
            TimeKind::Hours(x) => Some(*x),
            TimeKind::Minutes(x) => Some(*x),
            TimeKind::Seconds(x) => Some(*x),
            TimeKind::Milliseconds(x) => Some(*x),
            TimeKind::Unknown => None,
        }
    }

    fn set_value(&mut self, value: f64) -> &Self {
        match &self {
            TimeKind::Hours(_) => *self = TimeKind::Hours(value),
            TimeKind::Minutes(_) => *self = TimeKind::Minutes(value),
            TimeKind::Seconds(_) => *self = TimeKind::Seconds(value),
            TimeKind::Milliseconds(_) => *self = TimeKind::Milliseconds(value),
            TimeKind::Unknown => *self = TimeKind::Unknown,
        }
        self
    }

    fn as_standard_unit(&mut self) -> &mut Self {
        match &self {
            TimeKind::Hours(x) => { *self = TimeKind::Seconds({ *x * 60.0 * 60.0 }) }
            TimeKind::Minutes(x) => { *self = TimeKind::Seconds({ *x * 60.0 }) }
            TimeKind::Seconds(x) => { *self = TimeKind::Seconds(*x) }
            TimeKind::Milliseconds(x) => { *self = TimeKind::Seconds(*x / 1000.0) }
            TimeKind::Unknown => { *self = TimeKind::Unknown }
        }
        self
    }

    fn clone(&self) -> Self where Self: Sized {
        match self {
            TimeKind::Hours(x) => TimeKind::Hours(*x),
            TimeKind::Minutes(x) => TimeKind::Minutes(*x),
            TimeKind::Seconds(x) => TimeKind::Seconds(*x),
            TimeKind::Milliseconds(x) => TimeKind::Milliseconds(*x),
            TimeKind::Unknown => TimeKind::Unknown
        }
    }
}

impl fmt::Display for TimeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TimeKind::Hours(x) => { write!(f, "( Hours: {} )", x) }
            TimeKind::Minutes(x) => { write!(f, "( Minutes: {} )", x) }
            TimeKind::Seconds(x) => { write!(f, "( Seconds: {} )", x) }
            TimeKind::Milliseconds(x) => { write!(f, "( Milliseconds: {} )", x) }
            TimeKind::Unknown => { write!(f, "( Unknown )") }
        }
    }
}
