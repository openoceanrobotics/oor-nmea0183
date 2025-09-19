pub mod error;
pub mod gga;
pub mod hdm;
pub mod hdt;
pub mod ilt;
pub mod mwv;
pub mod pgilt;
pub mod seaview;
pub mod xdr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitsOfSpeed {
    KilometersPerHour,
    MetresPerSecond,
    Knots,
    StatuteMilesPerHour,
}

impl UnitsOfSpeed {
    fn from_char(c: &str) -> Option<Self> {
        match c {
            "K" => Some(Self::KilometersPerHour),
            "M" => Some(Self::MetresPerSecond),
            "N" => Some(Self::Knots),
            "S" => Some(Self::StatuteMilesPerHour),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitsOfMeasurement {
    Amperes,
    Bar,
    Celsius,
    Degrees,
    Hertz,
    Kelvin,
    LitresPerSecond,
    Meter,
    Newton,
    PartsPerThousand,
    Percent,
    Volts,
}

impl UnitsOfMeasurement {
    fn from_char(c: &str) -> Option<Self> {
        match c {
            "A" => Some(Self::Amperes),
            "B" => Some(Self::Bar),
            "C" => Some(Self::Celsius),
            "D" => Some(Self::Degrees),
            "H" => Some(Self::Hertz),
            "I" => Some(Self::LitresPerSecond),
            "K" => Some(Self::Kelvin),
            "M" => Some(Self::Meter),
            "N" => Some(Self::Newton),
            "P" => Some(Self::Percent),
            "S" => Some(Self::PartsPerThousand),
            "V" => Some(Self::Volts),
            _ => None,
        }
    }
}

/// Some messages can contain multiple types of transducer data.  It makes treanslation a bit of a pain in
/// the ass.  Here, we return a struct containing optional data from each possible transducer type
/// - one of them will have what you're looking for.  Right now we only support A and C.
///   A   Angular displacement
///   C   Temperature
///   D   Depth
///   F   Frequency
///   H   Humidity
///   N   Force
///   P   Pressure
///   R   Flow
#[derive(Debug, Clone)]
pub enum TransducerReading {
    AngularDisplacement(Reading),
    Temperature(Reading),
    Depth(Reading),
    Frequency(Reading),
    Humidity(Reading),
    Force(Reading),
    Pressure(Reading),
    Flow(Reading),
}

impl TransducerReading {
    fn from_nmea(c: &str, reading: Reading) -> Option<Self> {
        match c {
            "A" => Some(Self::AngularDisplacement(reading)),
            "C" => Some(Self::Temperature(reading)),
            "D" => Some(Self::Depth(reading)),
            "F" => Some(Self::Frequency(reading)),
            "H" => Some(Self::Humidity(reading)),
            "N" => Some(Self::Force(reading)),
            "P" => Some(Self::Pressure(reading)),
            "R" => Some(Self::Flow(reading)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Reading {
    pub reading: Option<f32>,
    pub units: Option<UnitsOfMeasurement>,
    pub name: Option<String>,
}
