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
pub mod illuminance;


pub trait UnitOfMeasureValueKind : Clone + Sized{
    fn get_value(&self) -> Option<f64>;
    fn set_value(&mut self, value: f64) -> &Self;
    fn as_standard_unit(&mut self) -> &Self;
}


