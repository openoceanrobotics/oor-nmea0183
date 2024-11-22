use super::error::ConvertNMEA0183Error;
use crate::Nmea;

#[derive(Debug, Clone)]
pub struct Mwv {
    pub talker_id: String,
    pub message_id: String,
    pub wind_direction: i32,
    pub wind_dir_type: WindDirectionType,
    pub wind_speed: f32,
    pub wind_speed_units: SpeedUnits,
    pub acceptable: AcceptableMeasurement,
}

impl TryFrom<Nmea> for Mwv {
    type Error = ConvertNMEA0183Error;
    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        let mwv = Mwv {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            wind_direction: nmea.fields[0].parse::<i32>()?,
            wind_dir_type: match nmea.fields[1].as_str() {
                "R" => WindDirectionType::Relative,
                "T" => WindDirectionType::True,
                field => return Err(ConvertNMEA0183Error::ConvertToEnumError(field.to_string())),
            },
            wind_speed: nmea.fields[2].parse::<f32>()?,
            wind_speed_units: match nmea.fields[3].as_str() {
                "N" => SpeedUnits::Knots,
                field => return Err(ConvertNMEA0183Error::ConvertToEnumError(field.to_string())),
            },
            acceptable: match nmea.fields[4].as_str() {
                "A" => AcceptableMeasurement::Acceptable,
                "V" => AcceptableMeasurement::Void,
                field => return Err(ConvertNMEA0183Error::ConvertToEnumError(field.to_string())),
            },
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
pub enum SpeedUnits {
    Knots,
}

#[derive(Debug, Clone)]
pub enum AcceptableMeasurement {
    Acceptable,
    Void,
}

