use crate::metrics::uom::UnitOfMeasureValueKind;

#[derive(Debug, Copy, Clone)]
pub enum TimeKind {
    Hours(f64),
    Minutes(f64),
    Seconds(f64),
    Milliseconds(f64),
}

impl UnitOfMeasureValueKind for TimeKind {
    fn get_value(&self) -> Option<f64> {
        match &self {
            TimeKind::Hours(x) => Some(*x),
            TimeKind::Minutes(x) => Some(*x),
            TimeKind::Seconds(x) => Some(*x),
            TimeKind::Milliseconds(x) => Some(*x),
        }
    }

    fn set_value(&mut self, value: f64) -> &Self {
        match &self {
            TimeKind::Hours(_) => *self = TimeKind::Hours(value),
            TimeKind::Minutes(_) => *self = TimeKind::Minutes(value),
            TimeKind::Seconds(_) => *self = TimeKind::Seconds(value),
            TimeKind::Milliseconds(_) => *self = TimeKind::Milliseconds(value),
        }
        self
    }

    fn as_standard_unit(&mut self) -> &Self {
        match &self {
            TimeKind::Hours(x) => { *self = TimeKind::Seconds({ *x / 60.0 / 60.0 }) }
            TimeKind::Minutes(x) => { *self = TimeKind::Seconds({ *x / 60.0 }) }
            TimeKind::Seconds(x) => { *self = TimeKind::Seconds(*x) }
            TimeKind::Milliseconds(x) => { *self = TimeKind::Seconds(*x * 1000.0) }
        }
        self
    }
}