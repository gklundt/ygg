#[derive(Debug)]
pub enum IlluminanceKind {
    Candelas(f64),
    Lumen(f64),
    Lux(f64),
    Unknown,
}