use crate::{sentences::error::ParseNMEA0183Error, Nmea};

/// Summary message (`$PSVSW`) from the SVS-603HR.
///
/// This message cannot be parsed because it is `dynamic`.  This means that  
/// empty parameters with extra commas to define each field in the NMEA0183 are not used and
/// instead the data in the message is defined by setting a bitmask field on the device.  
/// This makes it extremly difficult to parse in this library.
///
/// Format:
/// ```text
/// $PSVSW,0.570,2.560,311.141,311.142,3.872,1.559,0.118,3.824,12.6,23.8,SVS-603HR,2023-01-12 12:26:27,255*1A
/// ```
#[derive(Debug, Clone)]
pub struct Svsw {
    pub talker_id: String,
    pub message_id: String,

    pub params: Vec<String>,
}

impl TryFrom<Nmea> for Svsw {
    type Error = ParseNMEA0183Error;

    fn try_from(nmea: Nmea) -> Result<Self, Self::Error> {
        Ok(Svsw {
            talker_id: nmea.talker_id,
            message_id: nmea.message_id,
            params: nmea.fields,
        })
    }
}
