#[derive(Debug)]
pub enum PressureKind {
    PSI(f64),
    Pascal(f64),
    Atmosphere(f64),
    Torr(f64),
    NewtonPerSquareMeter(f64),
}

#[derive(Debug)]
pub enum FrequencyKind {
    Hertz(f64),
    Kilohertz(f64),
    Megahertz(f64),
    Gigahertz(f64),
}

#[derive(Debug)]
pub enum AccountKind {
    USDollars(f64),
    UKPounds(f64),
    DEMarks(f64),
    MXPeso(f64),
}

#[derive(Debug)]
pub enum WeightKind {
    Pounds(f64),
    KiloMetre(f64),
    Stones(f64),
    KiloPounds(f64),
    Newtons(f64),
}

#[derive(Debug)]
pub enum DistanceKind {
    Feet(f64),
    Miles(f64),
    Meters(f64),
    Inches(f64),
}

#[derive(Debug)]
pub enum TimeKind {
    Hours(f64),
    Minutes(f64),
    Seconds(f64),
    Milliseconds(f64),
}

#[derive(Debug)]
pub enum AngularKind {
    Degrees(f64),
    Radians(f64),
}

#[derive(Debug)]
pub enum TemperatureKind {
    Celsius(f64),
    Fahrenheit(f64),
    Rankine(f64),
}

#[derive(Debug)]
pub enum VolumeKind {
    CubicMeters(f64),
    CubicFeet(f64),
    CubicInches(f64),
    CubicCentimeters(f64),
}

#[derive(Debug)]
pub enum IlluminanceKind {
    Candelas(f64),
    Lumen(f64),
    Lux(f64),
}

//#[derive(Debug)]
//pub struct ResourceModifier {
//    name: String,
//    unique_identifier: UUID,
//    measurement: Measurement,
//    aggregation_group: i32,
//    is_quantized: bool,
//    quantum: f64,
//}