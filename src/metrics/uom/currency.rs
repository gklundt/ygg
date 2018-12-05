#[derive(Debug)]
pub enum AccountKind {
    USDollars(f64),
    UKPounds(f64),
    DEMarks(f64),
    MXPeso(f64),
    Unknown,
}
