#[derive(Debug)]
pub enum PressureKind {
    PSI(f64),
    Pascal(f64),
    Atmosphere(f64),
    Torr(f64),
    NewtonPerSquareMeter(f64),
}
