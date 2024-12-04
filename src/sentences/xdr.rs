use super::error::ConvertNMEA0183Error;
use crate::Nmea;

#[derive(Debug, Clone)]
pub struct Xdr {
    pub talker_id: String,
    pub message_id: String,
    pub readings: Vec<Option<TransducerType>>,
}

impl TryFrom<Nmea> for Xdr {
    type Error = ConvertNMEA0183Error;
    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        Ok(Xdr {
            talker_id: nmea.talker_id.clone(),
            message_id: nmea.message_id.clone(),
            readings: to_readings(nmea.fields),
        })
    }
}

pub fn to_readings(fields: Vec<String>) -> Vec<Option<TransducerType>> {
    let mut readings: Vec<Option<TransducerType>> = vec![];
    for i in (0..fields.len()).step_by(4) {
        let reading = Reading {
            reading: fields[i + 1].parse::<f32>().ok(),
            units: UnitsOfMeasurement::from_char(&fields[i + 2]),
            name: Some(fields[i + 3].clone()),
        };
        readings.push(TransducerType::from_nmea(&fields[i], reading));
    }
    readings
}

#[derive(Debug, Clone)]
pub struct Reading {
    pub reading: Option<f32>,
    pub units: Option<UnitsOfMeasurement>,
    pub name: Option<String>,
}

/// XDR can contain multiple types of transducer data.  It makes treanslation a bit of a pain in
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
pub enum TransducerType {
    AngularDisplacement(Reading),
    Temperature(Reading),
    Depth(Reading),
    Frequency(Reading),
    Humidity(Reading),
    Force(Reading),
    Pressure(Reading),
    Flow(Reading),
}

impl TransducerType {
    fn from_nmea(c: &str, reading: Reading) -> Option<Self> {
        match c {
            "A" => Some(TransducerType::AngularDisplacement(reading)),
            "C" => Some(TransducerType::Temperature(reading)),
            "D" => Some(TransducerType::Depth(reading)),
            "F" => Some(TransducerType::Frequency(reading)),
            "H" => Some(TransducerType::Humidity(reading)),
            "N" => Some(TransducerType::Force(reading)),
            "P" => Some(TransducerType::Pressure(reading)),
            "R" => Some(TransducerType::Flow(reading)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitsOfMeasurement {
    Bar,
    Celsius,
    Degrees,
    Meter,
    Newton,
    Percent,
}

impl UnitsOfMeasurement {
    fn from_char(c: &str) -> Option<Self> {
        match c {
            "B" => Some(UnitsOfMeasurement::Bar),
            "C" => Some(UnitsOfMeasurement::Celsius),
            "D" => Some(UnitsOfMeasurement::Degrees),
            "M" => Some(UnitsOfMeasurement::Meter),
            "N" => Some(UnitsOfMeasurement::Newton),
            "P" => Some(UnitsOfMeasurement::Percent),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_readings() {
        let fields = vec![
            "C", "+023.9", "C", "TEMP", "P", "1.0243", "B", "PRESS", "H", "039", "P", "RH",
        ];
        let fields = fields.into_iter().map(|v| v.to_string()).collect();
        let readings = to_readings(fields);

        assert_eq!(readings.len(), 3);
    }
}
