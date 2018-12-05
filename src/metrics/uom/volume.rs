#[derive(Debug)]
pub enum VolumeKind {
    CubicMeters(f64),
    CubicFeet(f64),
    CubicInches(f64),
    CubicCentimeters(f64),
    Unknown,
}