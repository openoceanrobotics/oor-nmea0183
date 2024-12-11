use super::error::ParseNMEA0183Error;
use crate::Nmea;

/// Represents the `$GPGGA` (Global Positioning System Fix Data) NMEA 0183 sentence.
///
/// This struct provides detailed information about the GPS fix, including the location, time, 
/// quality of the fix, and additional positioning details.
///
/// ### Fields:
/// - `talker_id`: The talker ID of the sentence (e.g., `GP` for GPS).
/// - `message_id`: The message ID, typically `GGA`.
/// - `fix_time`: Optional floating-point value representing the UTC time in `hhmmss.sss` format (e.g., `161229.487` for 16:12:29.487).
/// - `latitude`: Optional floating-point value representing the latitude in `ddmm.mmmm` format (e.g., `3723.2475` for 37°23.2475').
/// - `longitude`: Optional floating-point value representing the longitude in `ddmm.mmmm` format (e.g., `12158.3416` for 121°58.3416').
/// - `fix_quality`: Optional `FixQuality` enum indicating the quality of the GPS fix:
///   - `FixNotValid`: Fix not valid.
///   - `GpsFix`: Standard GPS fix.
///   - `DifferentialGpsFix`: Differential GPS fix.
///   - `NotApplicable`: Not applicable.
///   - `RtkFixed`: Real-Time Kinematic (RTK) fixed solution.
///   - `RtkFloat`: Real-Time Kinematic (RTK) float solution.
///   - `InsDeadReckoning`: Inertial Navigation System dead reckoning.
/// - `num_satellites`: Optional integer representing the number of satellites used (e.g., `07` for 7 satellites).
/// - `hdop`: Optional floating-point value representing the horizontal dilution of precision (e.g., `1.0`).
/// - `altitude_msl`: Optional floating-point value representing the altitude relative to mean sea level in meters (e.g., `9.0`).
/// - `geoid_separation`: Optional floating-point value representing the geoid separation in meters.
/// - `differential_age`: Optional floating-point value representing the age of differential corrections in seconds.
/// - `differential_gps_reference_station_id`: Optional integer representing the ID of the differential GPS reference station.
///
/// ### Example NMEA Sentence:
/// - `$GPGGA,161229.487,3723.2475,N,12158.3416,W,1,07,1.0,9.0,M, , , ,0000*18<CR><LF>`
///
/// ### Conversion:
/// Implements [`TryFrom<Nmea>`] to parse the `$GPGGA` sentence into a `Gga` struct.
/// The conversion extracts all relevant fields and parses them into the corresponding struct members.
#[derive(Debug, Clone)]
pub struct Gga {
    pub talker_id: String,
    pub message_id: String,
    pub fix_time: Option<f32>, // Convert to a time
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub fix_quality: Option<FixQuality>,
    pub num_satellites: Option<u32>,
    pub hdop: Option<f32>,
    pub altitude_msl: Option<f32>,
    pub geoid_separation: Option<f32>,
    pub differential_age: Option<f32>,
    pub differential_gps_reference_station_id: Option<u32>,
}

impl TryFrom<Nmea> for Gga {
    type Error = ParseNMEA0183Error;
    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        let mwv = Gga {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            fix_time: nmea.fields[0].parse::<f32>().ok(),
            latitude: nmea.fields[1].parse::<f64>().ok(),
            longitude: nmea.fields[3].parse::<f64>().ok(),
            fix_quality: Some(match nmea.fields[5].parse::<u8>()? {
                0 => FixQuality::FixNotValid,
                1 => FixQuality::GpsFix,
                2 => FixQuality::DifferentialGpsFix,
                3 => FixQuality::NotApplicable,
                4 => FixQuality::RtkFixed,
                5 => FixQuality::RtkFloat,
                6 => FixQuality::InsDeadReckoning,
                field => return Err(ParseNMEA0183Error::ConvertToEnumError(field.to_string())),
            }),
            num_satellites: nmea.fields[6].parse::<u32>().ok(),
            hdop: nmea.fields[7].parse::<f32>().ok(),
            altitude_msl: nmea.fields[8].parse::<f32>().ok(),
            geoid_separation: nmea.fields[10].parse::<f32>().ok(),
            differential_age: nmea.fields[12].parse::<f32>().ok(),
            differential_gps_reference_station_id: nmea.fields[13].parse::<u32>().ok(),
        };
        Ok(mwv)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixQuality {
    FixNotValid,
    GpsFix,
    DifferentialGpsFix,
    NotApplicable,
    RtkFixed,
    RtkFloat,
    InsDeadReckoning,
}
