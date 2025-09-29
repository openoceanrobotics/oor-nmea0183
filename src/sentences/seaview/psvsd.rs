use crate::{sentences::error::ParseNMEA0183Error, Nmea};

/// Wave directional energy spectrum telemetry (`$PSVSD`) from the SVS-603HR.
///
/// This sentence provides the **directional wave energy spectrum** as an array of
/// spectral energy densities for each configured frequency bin.
///
/// Format:
/// ```text
/// $PSVSD,E1,E2,...*CS
/// ```
/// - `Ei`: direcitonal spectral energy density for frequency bin
#[derive(Debug, Clone)]
pub struct Svsd {
    pub talker_id: String,
    pub message_id: String,

    /// Period for this direcitonal spectrum output
    pub period: f32,
    /// Energy values (ESmag) for each of N angles in this period.
    pub esmag: Vec<f32>,
}

impl TryFrom<Nmea> for Svsd {
    type Error = ParseNMEA0183Error;

    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        if nmea.fields.len() < 2 {
            return Err(ParseNMEA0183Error::MissingFields(2));
        }

        let bin_fields = &nmea.fields[1..nmea.fields.len()];
        let mut esmag = Vec::with_capacity(bin_fields.len());
        for field in bin_fields.iter() {
            let value = field.parse::<f32>()?;
            esmag.push(value);
        }

        Ok(Svsd {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            period: nmea.fields[0].parse::<f32>()?,
            esmag,
        })
    }
}
