#[derive(Debug)]
pub enum FrequencyKind {
    Hertz(f64),
    Kilohertz(f64),
    Megahertz(f64),
    Gigahertz(f64),
    Unknown,
}
