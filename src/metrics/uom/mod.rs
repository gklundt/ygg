pub mod distance;
pub mod pressure;
pub mod frequency;
pub mod currency;
pub mod weight;
pub mod time;
pub mod velocity;
pub mod angles;
pub mod temperature;
pub mod position;
pub mod volume;
pub mod Illuminance;

pub trait UnitOfMeasureValueKind {
    fn get_value(&self) -> Option<f64>;
    fn set_value(&mut self, value: f64) -> &Self;
    fn to_si(&self) -> Self;
}


