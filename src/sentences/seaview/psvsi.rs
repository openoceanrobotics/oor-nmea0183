use crate::{sentences::error::ParseNMEA0183Error, Nmea};

/// Identity report (`$PSVSI`) from the SVS-603HR.
///
/// Format:
/// ```text
/// $PSVSI,WINDMILL*73
/// ```
#[derive(Debug, Clone)]
pub struct Svsi {
    pub talker_id: String,
    pub message_id: String,

    /// Identity of the Seaview SVS-603HR device
    pub identity: String,
}

impl TryFrom<Nmea> for Svsi {
    type Error = ParseNMEA0183Error;

    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        if nmea.fields.is_empty() {
            return Err(ParseNMEA0183Error::MissingFields(1));
        }

        Ok(Svsi {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            identity: nmea.fields[0].clone(),
        })
    }
}
