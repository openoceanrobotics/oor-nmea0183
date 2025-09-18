use super::error::ParseNMEA0183Error;
use crate::Nmea;
use uom::si::{
    acceleration::meter_per_second_squared,
    angle::degree,
    angular_velocity::degree_per_second,
    f32::{Acceleration, Angle, AngularVelocity},
};

/// Dynamic motion telemetry (`$PSVDY`) from the SVS-603HR.
///
/// This sentence provides **instantaneous IMU data** at the device’s sample rate,
/// including raw accelerations, angular rates, attitude angles, and resolved
/// accelerations in the North-East-Up (NEU) frame.
///
/// Format:
/// ```text
/// $PSVDY,accX,accY,accZ,gyrp,gyrq,gyrr,angH,angP,angR,accN,accE,accU,index*CS
/// ```
#[derive(Debug, Clone)]
pub struct Svdy {
    pub talker_id: String,
    pub message_id: String,

    /// X-axis acceleration in the **sensor frame** (m/s²).
    pub acc_x: Option<Acceleration>,
    /// Y-axis acceleration in the **sensor frame** (m/s²).
    pub acc_y: Option<Acceleration>,
    /// Z-axis acceleration in the **sensor frame** (m/s²).
    pub acc_z: Option<Acceleration>,

    /// Angular rate about the **X-axis** (deg/s).
    pub gyr_p: Option<AngularVelocity>,
    /// Angular rate about the **Y-axis** (deg/s).
    pub gyr_q: Option<AngularVelocity>,
    /// Angular rate about the **Z-axis** (deg/s).
    pub gyr_r: Option<AngularVelocity>,

    /// Heading angle (deg).
    pub heading: Option<Angle>,
    /// Pitch angle (deg).
    pub pitch: Option<Angle>,
    /// Roll angle (deg).
    pub roll: Option<Angle>,

    /// Acceleration resolved in the **North** direction (m/s²).
    pub acc_n: Option<Acceleration>,
    /// Acceleration resolved in the **East** direction (m/s²).
    pub acc_e: Option<Acceleration>,
    /// Acceleration resolved in the **Up** direction (m/s²).
    pub acc_u: Option<Acceleration>,

    /// Monotonic sample index for detecting gaps and ordering.
    pub index: Option<u32>,
}

impl TryFrom<Nmea> for Svdy {
    type Error = ParseNMEA0183Error;

    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        if nmea.fields.len() < 12 {
            return Err(ParseNMEA0183Error::MissingFields(12));
        }

        Ok(Svdy {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,

            acc_x: nmea.fields[0]
                .parse::<f32>()
                .ok()
                .map(|v| Acceleration::new::<meter_per_second_squared>(v)),
            acc_y: nmea.fields[1]
                .parse::<f32>()
                .ok()
                .map(|v| Acceleration::new::<meter_per_second_squared>(v)),
            acc_z: nmea.fields[2]
                .parse::<f32>()
                .ok()
                .map(|v| Acceleration::new::<meter_per_second_squared>(v)),

            gyr_p: nmea.fields[3]
                .parse::<f32>()
                .ok()
                .map(|v| AngularVelocity::new::<degree_per_second>(v)),
            gyr_q: nmea.fields[4]
                .parse::<f32>()
                .ok()
                .map(|v| AngularVelocity::new::<degree_per_second>(v)),
            gyr_r: nmea.fields[5]
                .parse::<f32>()
                .ok()
                .map(|v| AngularVelocity::new::<degree_per_second>(v)),

            heading: nmea.fields[6]
                .parse::<f32>()
                .ok()
                .map(|v| Angle::new::<degree>(v)),
            pitch: nmea.fields[7]
                .parse::<f32>()
                .ok()
                .map(|v| Angle::new::<degree>(v)),
            roll: nmea.fields[8]
                .parse::<f32>()
                .ok()
                .map(|v| Angle::new::<degree>(v)),

            acc_n: nmea.fields[9]
                .parse::<f32>()
                .ok()
                .map(|v| Acceleration::new::<meter_per_second_squared>(v)),
            acc_e: nmea.fields[10]
                .parse::<f32>()
                .ok()
                .map(|v| Acceleration::new::<meter_per_second_squared>(v)),
            acc_u: nmea.fields[11]
                .parse::<f32>()
                .ok()
                .map(|v| Acceleration::new::<meter_per_second_squared>(v)),

            index: nmea.fields[12].parse::<u32>().ok(),
        })
    }
}

