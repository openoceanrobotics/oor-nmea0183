use super::{error::ConvertNMEA0183Error, Reading, TransducerReading, UnitsOfMeasurement};
use crate::Nmea;

#[derive(Debug, Clone)]
pub struct Xdr {
    pub talker_id: String,
    pub message_id: String,
    pub readings: Vec<Option<TransducerReading>>,
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

pub fn to_readings(fields: Vec<String>) -> Vec<Option<TransducerReading>> {
    let mut readings: Vec<Option<TransducerReading>> = vec![];
    for i in (0..fields.len()).step_by(4) {
        let reading = Reading {
            reading: fields[i + 1].parse::<f32>().ok(),
            units: UnitsOfMeasurement::from_char(&fields[i + 2]),
            name: Some(fields[i + 3].clone()),
        };
        readings.push(TransducerReading::from_nmea(&fields[i], reading));
    }
    readings
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
