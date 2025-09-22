use crate::{sentences::error::ParseNMEA0183Error, Nmea};

/// A single frequency-bin entry from the wave energy spectrum.
#[derive(Debug, Clone)]
pub struct SpectrumBin {
    pub period: f32, // Period for the bin
    pub es_mag: f32, // Energy spectrum magnitude
    pub n_re: f32,   // North real component
    pub n_im: f32,   // North imaginary component
    pub e_re: f32,   // East real component
    pub e_im: f32,   // East imaginary component
    pub u_re: f32,   // Up real component
    pub u_im: f32,   // Up imaginary component
    pub u_mag: f32,  // Magnitude of up energy
    pub theta: f32,  // Radians

    // Fourier coefficients
    pub a1: f32,
    pub b1: f32,
    pub a2: f32,
    pub b2: f32,
}

/// Wave energy spectrum telemetry (`$PSVSS`) from the SVS-603HR.
///
/// Each sentence corresponds to a **single frequency bin** with multiple values,
/// whose order is defined by the preceding `$PSVSH` header.
///
/// Format:
/// ```text
/// $PSVSS,Period,ESmag,Nre,Nim,Ere,Eim,Ure,Uim,Umag,theta,a1,b1,a2,b2*CS
/// ```
#[derive(Debug, Clone)]
pub struct Svss {
    pub talker_id: String,
    pub message_id: String,

    /// The spectrum data for this frequency bin.
    pub bin: SpectrumBin,
}

impl TryFrom<Nmea> for Svss {
    type Error = ParseNMEA0183Error;

    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        if nmea.fields.len() < 14 {
            return Err(ParseNMEA0183Error::MissingFields(14));
        }

        let bin = SpectrumBin {
            period: nmea.fields[0].parse::<f32>()?,
            es_mag: nmea.fields[1].parse::<f32>()?,
            n_re: nmea.fields[2].parse::<f32>()?,
            n_im: nmea.fields[3].parse::<f32>()?,
            e_re: nmea.fields[4].parse::<f32>()?,
            e_im: nmea.fields[5].parse::<f32>()?,
            u_re: nmea.fields[6].parse::<f32>()?,
            u_im: nmea.fields[7].parse::<f32>()?,
            u_mag: nmea.fields[8].parse::<f32>()?,
            theta: nmea.fields[9].parse::<f32>()?,
            a1: nmea.fields[10].parse::<f32>()?,
            b1: nmea.fields[11].parse::<f32>()?,
            a2: nmea.fields[12].parse::<f32>()?,
            b2: nmea.fields[13].parse::<f32>()?,
        };

        Ok(Svss {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            bin,
        })
    }
}
