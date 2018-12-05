#[derive(Debug)]
pub enum VelocityKind {
    MilesPerHour(f64),
    KilometersPerHour(f64),
    FeetPerSecond(f64),
    MetersPerSecond(f64),
    Knot(f64),
    Unknown,
}
