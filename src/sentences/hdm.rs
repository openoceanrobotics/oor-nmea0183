use super::error::ParseNMEA0183Error;
use crate::Nmea;

/// Represents WIHDM a heading measurement with magnetic heading type.
///
/// Fields:
/// - `talker_id`: The talker ID identifying the source of the NMEA sentence.
/// - `message_id`: The message ID identifying the type of NMEA sentence.
/// - `heading`: The heading value in degrees, which may be `None` if not available.
/// - `heading_type`: The type of heading, either `Magnetic` or `True`.
#[derive(Debug, Clone)]
pub struct Hdm {
    pub talker_id: String,
    pub message_id: String,
    pub heading: Option<f32>,
    pub heading_type: Option<HeadingType>,
}

impl TryFrom<Nmea> for Hdm {
    type Error = ParseNMEA0183Error;
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
    type Error = ParseNMEA0183Error;
    fn try_from(heading_type: &str) -> Result<Self, Self::Error> {
        Ok(match heading_type {
            "M" => HeadingType::Magentic,
            "T" => HeadingType::True,
            _ => {
                return Err(ParseNMEA0183Error::ConvertToEnumError(
                    heading_type.to_string(),
                ))
            }
        })
    }
}
