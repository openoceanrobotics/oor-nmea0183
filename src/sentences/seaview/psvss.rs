use crate::{sentences::error::ParseNMEA0183Error, Nmea};
use uom::si::{diffusion_coefficient::square_meter_per_second, f32::DiffusionCoefficient};

/// Wave energy spectrum telemetry (`$PSVSS`) from the SVS-603HR.
///
/// This sentence provides the **wave energy spectrum** as an array of
/// spectral energy densities (m²/s) for each configured frequency bin.
///
/// Format:
/// ```text
/// $PSVSS,E1,E2,...,EN,index*CS
/// ```
/// - `Ei`: spectral energy density for frequency bin i (m²/s)
/// - `N`: number of bins (set by `FREQBINS`)
/// - `index`: processing window index
#[derive(Debug, Clone)]
pub struct Svss {
    pub talker_id: String,
    pub message_id: String,

    /// Spectral energy values for each frequency bin (m²/Hz).
    pub energy: Vec<DiffusionCoefficient>,
}

impl TryFrom<Nmea> for Svss {
    type Error = ParseNMEA0183Error;

    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        if nmea.fields.len() < 2 {
            return Err(ParseNMEA0183Error::MissingFields(2));
        }

        let bin_fields = &nmea.fields[..nmea.fields.len()];

        let mut energy = Vec::with_capacity(bin_fields.len());
        for field in bin_fields.iter() {
            let value = field.parse::<f32>()?;
            energy.push(DiffusionCoefficient::new::<square_meter_per_second>(value));
        }

        Ok(Svss {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            energy,
        })
    }
}
