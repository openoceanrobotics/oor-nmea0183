use super::error::ConvertNMEA0183Error;
use crate::Nmea;

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
    type Error = ConvertNMEA0183Error;
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
                field => return Err(ConvertNMEA0183Error::ConvertToEnumError(field.to_string())),
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

#[derive(Debug, Clone)]
pub enum FixQuality {
    FixNotValid,
    GpsFix,
    DifferentialGpsFix,
    NotApplicable,
    RtkFixed,
    RtkFloat,
    InsDeadReckoning,
}
