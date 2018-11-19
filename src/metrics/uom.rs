use std::f64::consts;
use std::fmt;

const METER_PER_FOOT: f64 = 0.3048;
const METER_PER_MILE: f64 = 1609.34;
const METER_PER_INCH: f64 = 0.0254;
const RAD_PER_DEG: f64 = consts::PI / 180.0;
const METER_PER_KILOMETER: f64 = 1000.0;
pub const EQUATORIAL_RADIUS_OF_EARTH: DistanceKind = DistanceKind::Kilometers(6378.0);
pub const POLAR_RADIUS_OF_EARTH: DistanceKind = DistanceKind::Kilometers(6357.0);


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
    Kilometers(f64),
    Inches(f64),
    Unknown,
}

impl Si for DistanceKind {
    fn to_si(&self) -> Self {
        match self {
            DistanceKind::Feet(ft) => DistanceKind::Meters(ft * METER_PER_FOOT),
            DistanceKind::Miles(mi) => DistanceKind::Meters(mi * METER_PER_MILE),
            DistanceKind::Kilometers(m) => DistanceKind::Meters(m * METER_PER_KILOMETER),
            DistanceKind::Meters(m) => DistanceKind::Meters(*m),
            DistanceKind::Inches(i) => DistanceKind::Meters(i * METER_PER_INCH),
            _ => DistanceKind::Unknown,
        }
    }
}

impl fmt::Display for DistanceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            DistanceKind::Feet(ft) => { write!(f, "( Feet: {} )", ft) }
            DistanceKind::Miles(mi) => { write!(f, "( Miles: {} )", mi) }
            DistanceKind::Kilometers(km) => { write!(f, "( Kilometers: {} )", km) }
            DistanceKind::Meters(m) => { write!(f, "( Meters: {} )", m) }
            DistanceKind::Inches(i) => { write!(f, "( Inches: {} )", i) }
            DistanceKind::Unknown => { write!(f, "( Unknown") }
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
    pub fn to_kilometers(&self) -> Self {
        let meter: DistanceKind = self.to_si();
        let mut i: f64 = 0.0;
        if let DistanceKind::Meters(m) = meter { i = m / METER_PER_KILOMETER };
        DistanceKind::Kilometers(i)
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
pub enum VelocityKind {
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


impl fmt::Display for AngularKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AngularKind::Degrees(deg) => { write!(f, "( Degrees: {} )", deg) }
            AngularKind::Radians(rad) => { write!(f, "( Radians: {} )", rad) }
        }
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

impl fmt::Display for PositionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            PositionKind::TwoDimensionEuclidean { x, y } => { write!(f, "( x: {}, y: {} )", x, y) }
            PositionKind::ThreeDimensionEuclidean { x, y, z } => { write!(f, "( x: {}, y: {}, z: {} )", x, y, z) }
            PositionKind::TwoDimensionGeo { lat, lng } => { write!(f, "( lat: {}, long: {} )", lat, lng) }
            PositionKind::Polar { radial, theta } => { write!(f, "( radius: {}, theta: {} )", radial, theta) }
            PositionKind::Spherical { radial, polar, azimuth } => { write!(f, "( radial: {}, polar: {}, azimuth: {} )", radial, polar, azimuth) }
            PositionKind::Unknown => { write!(f, "( Unknown Position )") }
        }
    }
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
