use std::f64::consts;

const METER_PER_FOOT: f64 = 0.3048;
const METER_PER_MILE: f64 = 1609.34;
const METER_PER_INCH: f64 = 0.0254;
const RAD_PER_DEG: f64 = consts::PI / 180.0;

pub trait Si {
    fn to_si(&self) -> Self;
}

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
    Unknown,
}

impl Si for DistanceKind {
    fn to_si(&self) -> Self {
        match self {
            DistanceKind::Feet(ft) => DistanceKind::Meters(ft * METER_PER_FOOT),
            DistanceKind::Miles(mi) => DistanceKind::Meters(mi * METER_PER_MILE),
            DistanceKind::Meters(m) => DistanceKind::Meters(*m),
            DistanceKind::Inches(i) => DistanceKind::Meters(i * METER_PER_INCH),
            _ => DistanceKind::Unknown,
        }
    }
}

impl DistanceKind {
    pub fn to_feet(&self) -> Self {
        let meter: DistanceKind = self.to_si();
        let mut i: f64 = 0.0;
        if let DistanceKind::Meters(m) = meter { i = m / METER_PER_FOOT };
        DistanceKind::Feet(i)
    }
    pub fn to_miles(&self) -> Self {
        let meter: DistanceKind = self.to_si();
        let mut i: f64 = 0.0;
        if let DistanceKind::Meters(m) = meter { i = m / METER_PER_MILE };
        DistanceKind::Miles(i)
    }
    pub fn to_inches(&self) -> Self {
        let meter: DistanceKind = self.to_si();
        let mut i: f64 = 0.0;
        if let DistanceKind::Meters(m) = meter { i = m / METER_PER_INCH };
        DistanceKind::Inches(i)
    }
}

#[derive(Debug)]
pub enum TimeKind {
    Hours(f64),
    Minutes(f64),
    Seconds(f64),
    Milliseconds(f64),
}

#[derive(Debug)]
pub enum SpeedKind {
    MilesPerHour(f64),
    KilometersPerHour(f64),
    FeetPerSecond(f64),
    MetersPerSecond(f64),
    Knot(f64),
}



#[derive(Debug)]
pub enum AngularKind {
    Degrees(f64),
    Radians(f64),
}


impl Si for AngularKind {
    fn to_si(&self) -> Self {
        match self {
            AngularKind::Radians(r) => AngularKind::Radians(*r),
            AngularKind::Degrees(d) => AngularKind::Radians(d * RAD_PER_DEG),
        }
    }
}

impl AngularKind {
    pub fn to_deg(&self) -> Self {
        let si: AngularKind = self.to_si();
        let mut i: f64 = 0.0;
        if let AngularKind::Radians(r) = si { i = r / RAD_PER_DEG };
        AngularKind::Degrees(i)
    }

    pub fn to_deg_rem(&self) -> Self {
        let si: AngularKind = self.to_si();
        let mut i: f64 = 0.0;
        if let AngularKind::Radians(r) = si { i = (r / RAD_PER_DEG) % 360.0 };
        AngularKind::Degrees(i)
    }
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

#[derive(Debug)]
pub enum PositionKind {
    TwoDimensionEuclidean { x: DistanceKind, y: DistanceKind },
    ThreeDimensionEuclidean { x: DistanceKind, y: DistanceKind, z: DistanceKind },
    TwoDimensionGeo { lat: AngularKind, lng: AngularKind },
    Spherical { radial: DistanceKind, polar: AngularKind, azimuth: AngularKind },
    Polar { radial: DistanceKind, theta: AngularKind },
    Unknown,
}

impl Si for PositionKind {
    fn to_si(&self) -> Self {
        match self {
            PositionKind::TwoDimensionEuclidean { x, y } => PositionKind::TwoDimensionEuclidean { x: x.to_si(), y: y.to_si() },
            PositionKind::ThreeDimensionEuclidean { x, y, z } => PositionKind::ThreeDimensionEuclidean { x: x.to_si(), y: y.to_si(), z: z.to_si() },
            PositionKind::TwoDimensionGeo { lat, lng } => PositionKind::TwoDimensionGeo { lat: lat.to_si(), lng: lng.to_si() },
            PositionKind::Polar { radial, theta } => PositionKind::Polar { radial: radial.to_si(), theta: theta.to_si() },
            PositionKind::Spherical { radial, polar, azimuth } => PositionKind::Spherical { radial: radial.to_si(), polar: polar.to_si(), azimuth: azimuth.to_si() },
            PositionKind::Unknown => PositionKind::Unknown,
        }
    }
}
