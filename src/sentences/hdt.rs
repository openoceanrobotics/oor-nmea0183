use super::{error::ConvertNMEA0183Error, hdm::HeadingType};
use crate::Nmea;

#[derive(Debug, Clone)]
pub struct Hdt {
    pub talker_id: String,
    pub message_id: String,
    pub heading: Option<f32>,
    pub heading_type: Option<HeadingType>,
}

impl TryFrom<Nmea> for Hdt {
    type Error = ConvertNMEA0183Error;
    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        Ok(Hdt {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            heading: nmea.fields[0].parse::<f32>().ok(),
            heading_type: nmea.fields[1].as_str().try_into().ok(),
        })
    }
}
