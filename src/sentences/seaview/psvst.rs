use crate::{sentences::error::ParseNMEA0183Error, Nmea};
use chrono::NaiveDateTime;

/// Displacement time series timestamp (`$PSVST`) from the SVS-603HR.
///
/// This sentence provides the **UTC timestamp** associated with
/// a displacement record block. Used to align `$PSVSI`, `$PSVSH`, and
/// `$PSVSV` messages for the same sample window.
///
/// Format:
/// ```text
/// $PSVST,YYYY-MM-DD HH:MM:SS*CS
/// ```
#[derive(Debug, Clone)]
pub struct Svst {
    pub talker_id: String,
    pub message_id: String,

    /// UTC timestamp parsed from the sentence.
    pub timestamp: Option<NaiveDateTime>,
}

impl TryFrom<Nmea> for Svst {
    type Error = ParseNMEA0183Error;

    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        if nmea.fields.is_empty() {
            return Err(ParseNMEA0183Error::MissingFields(1));
        }

        let timestamp = NaiveDateTime::parse_from_str(&nmea.fields[0], "%Y-%m-%d %H:%M:%S").ok();

        Ok(Svst {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            timestamp,
        })
    }
}
