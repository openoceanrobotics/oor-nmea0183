use super::error::ConvertNMEA0183Error;
use crate::Nmea;

#[derive(Debug, Clone)]
pub struct Hdm {
    pub talker_id: String,
    pub message_id: String,
    pub heading: Option<f32>,
    pub heading_type: Option<HeadingType>,
}

impl TryFrom<Nmea> for Hdm {
    type Error = ConvertNMEA0183Error;
    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        Ok(Hdm {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            heading: nmea.fields[0].parse::<f32>().ok(),
            heading_type: nmea.fields[1].as_str().try_into().ok(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum HeadingType {
    Magentic,
    True,
}

impl TryFrom<&str> for HeadingType {
    type Error = ConvertNMEA0183Error;
    fn try_from(heading_type: &str) -> Result<Self, Self::Error> {
        Ok(match heading_type {
            "M" => HeadingType::Magentic,
            "T" => HeadingType::True,
            _ => {
                return Err(ConvertNMEA0183Error::ConvertToEnumError(
                    heading_type.to_string(),
                ))
            }
        })
    }
}
