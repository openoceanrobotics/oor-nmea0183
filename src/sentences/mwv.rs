use super::{error::ParseNMEA0183Error, UnitsOfSpeed};
use crate::Nmea;

/// Represents the `$WIMWV` (Wind Instrument Mean Wind direction and Velocity) NMEA 0183 sentence.
///
/// ### Fields:
/// - `talker_id`: The talker ID of the sentence (e.g., `WI` for wind instrument).
/// - `message_id`: The message ID, usually `MWV`.
/// - `wind_direction_deg`: Optional integer value representing the wind direction in degrees (e.g., `049` for 49°).
/// - `wind_dir_type`: Optional `WindDirectionType` indicating the type of wind direction measurement:
///   - `Relative`: Wind measurement relative to the instrument's orientation.
///   - `True`: Wind measurement corrected to true north using a compass or GPS.
/// - `wind_speed_knots`: Optional floating-point value representing the wind speed in knots (e.g., `000.03` for 0.03 knots).
/// - `acceptable`: Optional `AcceptableMeasurement` indicating whether the measurement is valid:
///   - `Acceptable`: The measurement is valid.
///   - `Void`: The measurement is invalid or void.
///
/// ### Example NMEA Sentences:
/// - `$WIMWV,049,R,000.03,N,A*03<CR><LF>`
/// - `$WIMWV,049,T,,N,A*18<CR><LF>`
///
/// ### Conversion:
/// Implements [`TryFrom<Nmea>`] to parse the `$WIMWV` sentence into an `Mwv` struct.
/// The conversion will fail if fields cannot be parsed or contain invalid values.
#[derive(Debug, Clone)]
pub struct Mwv {
    pub talker_id: String,
    pub message_id: String,
    pub wind_direction_deg: Option<i32>,
    pub wind_dir_type: Option<WindDirectionType>,
    pub wind_speed: Option<f32>,
    pub wind_speed_units: Option<UnitsOfSpeed>,
    pub acceptable: Option<AcceptableMeasurement>,
}

impl TryFrom<Nmea> for Mwv {
    type Error = ParseNMEA0183Error;
    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        let mwv = Mwv {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            wind_direction_deg: nmea.fields[0].parse::<i32>().ok(),
            wind_dir_type: Some(match nmea.fields[1].as_str() {
                "R" => WindDirectionType::Relative,
                "T" => WindDirectionType::True,
                field => return Err(ParseNMEA0183Error::ConvertToEnumError(field.to_string())),
            }),
            wind_speed: nmea.fields[2].parse::<f32>().ok(),
            wind_speed_units: UnitsOfSpeed::from_char(&nmea.fields[3]),

            acceptable: Some(match nmea.fields[4].as_str() {
                "A" => AcceptableMeasurement::Acceptable,
                "V" => AcceptableMeasurement::Void,
                field => return Err(ParseNMEA0183Error::ConvertToEnumError(field.to_string())),
            }),
        };
        Ok(mwv)
    }
}

#[derive(Debug, Clone)]
pub enum WindDirectionType {
    Relative,
    True,
}

#[derive(Debug, Clone)]
pub enum AcceptableMeasurement {
    Acceptable,
    Void,
}
