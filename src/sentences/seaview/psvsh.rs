use crate::{sentences::error::ParseNMEA0183Error, Nmea};

/// Nmea header messages (`$PSVSH`) from the SVS-603HR.
///
/// Format:
/// ```text
/// $PSVSH,Period,ESmag,Nre,Nim,Ere,Eim,Ure,Uim,Umag,theta,a1,b1,a2,b2*57
/// ```
#[derive(Debug, Clone)]
pub struct Svsh {
    pub talker_id: String,
    pub message_id: String,

    pub headers: Vec<String>,
}

impl TryFrom<Nmea> for Svsh {
    type Error = ParseNMEA0183Error;

    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        Ok(Svsh {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            headers: nmea.fields,
        })
    }
}
