#[derive(Debug)]
pub enum WeightKind {
    Pounds(f64),
    KiloMetre(f64),
    Stones(f64),
    KiloPounds(f64),
    Newtons(f64),
}
