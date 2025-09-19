use crate::{sentences::error::ParseNMEA0183Error, Nmea};

/// Wave directional energy spectrum telemetry (`$PSVSD`) from the SVS-603HR.
///
/// This sentence provides the **directional wave energy spectrum** as an array of
/// spectral energy densities (m²/Hz) for each configured frequency bin.
///
/// Format:
/// ```text
/// $PSVSD,E1,E2,...*CS
/// ```
/// - `Ei`: direcitonal spectral energy density for frequency bin i (m²/Hz)
#[derive(Debug, Clone)]
pub struct Svsd {
    pub talker_id: String,
    pub message_id: String,

    /// Spectral energy values for each frequency bin (m²/Hz).
    pub directional_spectrum_bins: Vec<f32>,
}

impl TryFrom<Nmea> for Svsd {
    type Error = ParseNMEA0183Error;

    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        if nmea.fields.len() < 2 {
            return Err(ParseNMEA0183Error::MissingFields(2));
        }

        let bin_fields = &nmea.fields[..nmea.fields.len()];

        let mut directional_spectrum_bins = Vec::with_capacity(bin_fields.len());
        for field in bin_fields.iter() {
            let value = field.parse::<f32>()?;
            directional_spectrum_bins.push(value);
        }

        Ok(Svsd {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            directional_spectrum_bins,
        })
    }
}
