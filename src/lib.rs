pub mod sentences;
use pest::Parser;
use pest_derive::Parser;
use sentences::{error::ConvertNMEA0183Error, gga::Gga, hdm::Hdm, hdt::Hdt, mwv::Mwv, xdr::Xdr};

#[derive(Debug)]
pub enum Sentence {
    Unknown,
    Mwv(sentences::mwv::Mwv),
    Xdr(sentences::xdr::Xdr),
    Gga(sentences::gga::Gga),
    Hdm(sentences::hdm::Hdm),
    Hdt(sentences::hdt::Hdt),
    // Ilt(sentences::ilt::Ilt), // Propietary
}

#[derive(Parser)]
#[grammar = "nmea.pest"] // Relative path to the grammar file
pub struct NmeaPest;

#[derive(Debug)]
pub struct Nmea {
    pub talker_id: String,
    pub message_id: String,
    pub fields: Vec<String>,
}

pub struct NmeaParser {}
impl NmeaParser {
    pub fn parse(nmea_sentence: &str) -> Result<Sentence, ConvertNMEA0183Error> {
        let nmea = NmeaParser::to_nmea(nmea_sentence);
        Ok(match nmea.message_id.as_ref() {
            "MWV" => Sentence::Mwv(Mwv::try_from(nmea)?),
            "GGA" => Sentence::Gga(Gga::try_from(nmea)?),
            "XDR" => Sentence::Xdr(Xdr::try_from(nmea)?),
            "HDM" => Sentence::Hdm(Hdm::try_from(nmea)?),
            "HDT" => Sentence::Hdt(Hdt::try_from(nmea)?),
            _ => Sentence::Unknown,
        })
    }

    fn to_nmea(nmea_sentence: &str) -> Nmea {
        let parsed = NmeaPest::parse(Rule::NMEA, nmea_sentence)
            .unwrap()
            .next()
            .unwrap();

        let mut talker_id = String::default();
        let mut message_type = String::default();
        let mut fields = Vec::new();
        let mut checksum = None;

        for pair in parsed.into_inner() {
            // println!("{:?}", pair);
            match pair.as_rule() {
                Rule::talker_id => talker_id = pair.as_str().to_string(),
                Rule::message_type => message_type = pair.as_str().to_string(),
                Rule::fields => {
                    fields = pair
                        .as_str()
                        .split(',')
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>();
                }

                //TODO(@mattcairns): Verify checksum
                Rule::checksum => checksum = Some(pair.as_str().to_string()),
                _ => {}
            }
        }

        Nmea {
            talker_id,
            message_id: message_type,
            fields,
        }
    }
}

#[cfg(test)]
mod tests {
    use sentences::{
        gga::FixQuality,
        xdr::{TransducerType, UnitsOfMeasurement},
    };

    use super::*;

    #[test]
    fn test_mwv() {
        let input = "$WIMWV,049,R,000.03,N,A*03";
        let output = NmeaParser::parse(input).unwrap();
        match output {
            Sentence::Mwv(nmea) => {
                assert_eq!(nmea.talker_id, "WI");
                assert_eq!(nmea.message_id, "MWV");
            }
            _ => panic!("Expected Mwv"),
        };
        let input = "$WIMWV,180,T,000.11,N,A*02";
        let output = NmeaParser::parse(input).unwrap();
        match output {
            Sentence::Mwv(nmea) => {
                assert_eq!(nmea.talker_id, "WI");
                assert_eq!(nmea.message_id, "MWV");
            }
            _ => panic!("Expected Mwv"),
        }
    }

    #[test]
    fn test_xdr() {
        let input = "$WIXDR,C,+023.1,C,TEMP,P,0.9989,B,PRESS,H,040,P,RH*25";
        let output = NmeaParser::parse(input).unwrap();
        match output {
            Sentence::Xdr(nmea) => {
                assert_eq!(nmea.talker_id, "WI");
                assert_eq!(nmea.message_id, "XDR");
                if let Some(TransducerType::Temperature(r)) = &nmea.readings[0] {
                    assert_eq!(r.reading, Some(23.1));
                    assert_eq!(r.units, Some(UnitsOfMeasurement::Celsius));
                    assert_eq!(r.name, Some("TEMP".into()));
                }
                if let Some(TransducerType::Pressure(r)) = &nmea.readings[1] {
                    assert_eq!(r.reading, Some(0.9989));
                    assert_eq!(r.units, Some(UnitsOfMeasurement::Bar));
                    assert_eq!(r.name, Some("PRESS".into()));
                }
                if let Some(TransducerType::Pressure(r)) = &nmea.readings[2] {
                    assert_eq!(r.reading, Some(40.0));
                    assert_eq!(r.units, Some(UnitsOfMeasurement::Percent));
                    assert_eq!(r.name, Some("RH".into()));
                }
            }
            _ => panic!("Expected Xdr"),
        }
    }

    #[test]
    fn test_gga() {
        let input = "$GPGGA,113342.000,5045.7837,N,00132.4127,W,1,06,1.3,-10.2,M,47.8,M,,0000*56";
        let output = NmeaParser::parse(input).unwrap();
        match output {
            Sentence::Gga(nmea) => {
                assert_eq!(nmea.talker_id, "GP");
                assert_eq!(nmea.message_id, "GGA");
                assert_eq!(nmea.fix_time, Some(113342.000));
                assert_eq!(nmea.latitude, Some(5045.7837));
                assert_eq!(nmea.longitude, Some(00132.4127));
                assert_eq!(nmea.fix_quality, Some(FixQuality::GpsFix));
                assert_eq!(nmea.num_satellites, Some(6));
                assert_eq!(nmea.hdop, Some(1.3));
                assert_eq!(nmea.altitude_msl, Some(-10.2));
                assert_eq!(nmea.geoid_separation, Some(47.8));
                assert_eq!(nmea.differential_age, None);
                assert_eq!(nmea.differential_gps_reference_station_id, Some(0));
            }
            _ => panic!("Expected Gga"),
        }
    }

    #[test]
    fn test_gilt() {
        // let input = "$PGILT,A,+00,D,+01,D,+1,TILT*35";
        // let output = NmeaParser::parse(input).unwrap();
        // match output {
        //     Sentence::Gga(nmea) => {
        //         assert_eq!(nmea.talker_id, "GP");
        //         assert_eq!(nmea.message_id, "GGA");
        //         assert_eq!(nmea.fix_time, Some(113342.000));
        //         assert_eq!(nmea.latitude, Some(5045.7837));
        //         assert_eq!(nmea.longitude, Some(00132.4127));
        //         assert_eq!(nmea.fix_quality, Some(FixQuality::GpsFix));
        //         assert_eq!(nmea.num_satellites, Some(6));
        //         assert_eq!(nmea.hdop, Some(1.3));
        //         assert_eq!(nmea.altitude_msl, Some(-10.2));
        //         assert_eq!(nmea.geoid_separation, Some(47.8));
        //         assert_eq!(nmea.differential_age, None);
        //         assert_eq!(nmea.differential_gps_reference_station_id, Some(0));
        //     }
        //     _ => panic!("Expected Gga"),
        // }
    }

    #[test]
    fn test_parse_nmea_0183() {
        let input = "$PGILT,A,+00,D,+01,D,+1,TILT*35";
        let nmea = NmeaParser::to_nmea(input);
        assert_eq!(nmea.talker_id, "PG");
        assert_eq!(nmea.message_id, "ILT");
        assert_eq!(nmea.fields[0], "A");
        assert_eq!(nmea.fields[1], "+00");
        assert_eq!(nmea.fields[2], "D");
        assert_eq!(nmea.fields[3], "+01");
        assert_eq!(nmea.fields[4], "D");
        assert_eq!(nmea.fields[5], "+1");
        assert_eq!(nmea.fields[6], "TILT");
    }
}
