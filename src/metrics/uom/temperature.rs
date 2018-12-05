#[derive(Debug)]
pub enum TemperatureKind {
    Celsius(f64),
    Fahrenheit(f64),
    Rankine(f64),
    Unknown,
}