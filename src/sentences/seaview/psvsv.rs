use crate::{sentences::error::ParseNMEA0183Error, Nmea};
use uom::si::{
    f32::{Length, Time},
    length::meter,
    time::second,
};

/// Displacement (heave) (`$PSVSV`) from the SVS-603HR.
///
/// Format:
/// ```text
/// $PSVSV,0.000,0.024,0.037,-0.663*7C
/// ```
#[derive(Debug, Clone)]
pub struct Svsv {
    pub talker_id: String,
    pub message_id: String,

    pub time: Option<Time>,
    pub north: Option<Length>,
    pub east: Option<Length>,
    pub up: Option<Length>,
}

impl TryFrom<Nmea> for Svsv {
    type Error = ParseNMEA0183Error;

    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        if nmea.fields.len() < 4 {
            return Err(ParseNMEA0183Error::MissingFields(4));
        }

        Ok(Svsv {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            time: nmea.fields[0]
                .parse::<f32>()
                .ok()
                .map(|v| Time::new::<second>(v)),
            north: nmea.fields[1]
                .parse::<f32>()
                .ok()
                .map(|v| Length::new::<meter>(v)),
            east: nmea.fields[2]
                .parse::<f32>()
                .ok()
                .map(|v| Length::new::<meter>(v)),
            up: nmea.fields[3]
                .parse::<f32>()
                .ok()
                .map(|v| Length::new::<meter>(v)),
        })
    }
}
