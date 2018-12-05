use std::fmt::Debug;

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
pub mod quantity;


pub trait UnitOfMeasureValueKind: Debug {
    fn get_value(&self) -> Option<f64>;
    fn set_value(&mut self, value: f64) -> &Self where Self: Sized;
    fn as_standard_unit(&mut self) -> &mut Self where Self: Sized;
    fn clone(&self) -> Self where Self: Sized;
}



