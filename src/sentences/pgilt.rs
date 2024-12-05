use super::{error::ConvertNMEA0183Error, Reading, TransducerReading, UnitsOfMeasurement};
use crate::Nmea;

#[derive(Debug, Clone)]
pub struct Gilt {
    pub talker_id: String,
    pub message_id: String,
    pub x_tilt: Option<TransducerReading>,
    pub y_tilt: Option<TransducerReading>,
    pub z_orientation: Option<ZOrientation>,
    pub sensor_name: Option<String>,
}

impl TryFrom<Nmea> for Gilt {
    type Error = ConvertNMEA0183Error;
    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        Ok(Gilt {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            x_tilt: Some(TransducerReading::AngularDisplacement(Reading {
                reading: nmea.fields[1].parse::<f32>().ok(),
                units: Some(UnitsOfMeasurement::Degrees),
                name: None,
            })),
            y_tilt: Some(TransducerReading::AngularDisplacement(Reading {
                reading: nmea.fields[3].parse::<f32>().ok(),
                units: Some(UnitsOfMeasurement::Degrees),
                name: None,
            })),
            z_orientation: Some(ZOrientation::try_from(nmea.fields[5].parse::<i32>()?)?),
            sensor_name: Some(nmea.fields[6].to_string()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZOrientation {
    FacingUpright,
    FacingDownwards,
}

impl TryFrom<i32> for ZOrientation {
    type Error = ConvertNMEA0183Error;
    fn try_from(s: i32) -> Result<Self, Self::Error> {
        match s {
            1 => Ok(ZOrientation::FacingUpright),
            -1 => Ok(ZOrientation::FacingDownwards),
            _ => Err(ConvertNMEA0183Error::ConvertToEnumError(format!(
                "Failed to convert: {:?}",
                s
            ))),
        }
    }
}
