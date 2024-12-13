pub mod error;
pub mod gga;
pub mod hdm;
pub mod hdt;
pub mod ilt;
pub mod mwv;
pub mod pgilt;
pub mod xdr;

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
            "A" => Some(UnitsOfMeasurement::Amperes),
            "B" => Some(UnitsOfMeasurement::Bar),
            "C" => Some(UnitsOfMeasurement::Celsius),
            "D" => Some(UnitsOfMeasurement::Degrees),
            "H" => Some(UnitsOfMeasurement::Hertz),
            "I" => Some(UnitsOfMeasurement::LitresPerSecond),
            "K" => Some(UnitsOfMeasurement::Kelvin),
            "M" => Some(UnitsOfMeasurement::Meter),
            "N" => Some(UnitsOfMeasurement::Newton),
            "P" => Some(UnitsOfMeasurement::Percent),
            "S" => Some(UnitsOfMeasurement::PartsPerThousand),
            "V" => Some(UnitsOfMeasurement::Volts),
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
            "A" => Some(TransducerReading::AngularDisplacement(reading)),
            "C" => Some(TransducerReading::Temperature(reading)),
            "D" => Some(TransducerReading::Depth(reading)),
            "F" => Some(TransducerReading::Frequency(reading)),
            "H" => Some(TransducerReading::Humidity(reading)),
            "N" => Some(TransducerReading::Force(reading)),
            "P" => Some(TransducerReading::Pressure(reading)),
            "R" => Some(TransducerReading::Flow(reading)),
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

