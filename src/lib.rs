pub mod sentences;
use pest::Parser;
use pest_derive::Parser;
use sentences::{gga::Gga, mwv::Mwv};

#[derive(Debug)]
pub enum Sentence {
    Unknown,
    Mwv(sentences::mwv::Mwv),
    // Xdr(sentences::xdr::Xdr),
    Gga(sentences::gga::Gga),
    // ILT(sentences::ilt::Ilt),
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
    pub fn parse(nmea_sentence: &str) -> Sentence {
        let nmea = NmeaParser::to_nmea(nmea_sentence);
        match nmea.message_id.as_ref() {
            "MWV" => Sentence::Mwv(Mwv::try_from(nmea).unwrap()),
            "GGA" => Sentence::Gga(Gga::try_from(nmea).unwrap()),
            _ => Sentence::Unknown,
        }
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
    use super::*;

    #[test]
    fn test_mwv() {
        let input = "$WIMWV,049,R,000.03,N,A*03";
        let output = NmeaParser::parse(input);
        if let Sentence::Mwv(mwv) = output {
            assert_eq!(mwv.talker_id, "WI");
            assert_eq!(mwv.message_id, "MWV");
        }
        let input = "$WIMWV,180,T,000.11,N,A*02";
        let output = NmeaParser::parse(input);
        if let Sentence::Mwv(mwv) = output {
            assert_eq!(mwv.talker_id, "WI");
            assert_eq!(mwv.message_id, "MWV");
        }
    }

    #[test]
    fn test_gga() {
        let input = "$GPGGA,113342.000,5045.7837,N,00132.4127,W,1,06,1.3,-10.2,M,47.8,M,,0000*56";
        let output = NmeaParser::parse(input);
        println!("{:?}", output);
        if let Sentence::Gga(mwv) = output {
            assert_eq!(mwv.talker_id, "GP");
            assert_eq!(mwv.message_id, "GGA");
        }
    }

    #[test]
    fn test_parse_nmea_0183() {
        let input = "$WIMWV,049,R,000.03,N,A*03";
        let nmea = NmeaParser::to_nmea(input);
        assert_eq!(nmea.talker_id, "WI");
        assert_eq!(nmea.message_id, "MWV");
        assert_eq!(nmea.fields[0], "049");
        assert_eq!(nmea.fields[1], "R");
        assert_eq!(nmea.fields[2], "000.03");
        assert_eq!(nmea.fields[3], "N");
        assert_eq!(nmea.fields[4], "A");

        let input = "$WIMWV,180,T,000.11,N,A*02";
        let nmea = NmeaParser::to_nmea(input);
        assert_eq!(nmea.talker_id, "WI");
        assert_eq!(nmea.message_id, "MWV");
        assert_eq!(nmea.fields[0], "180");
        assert_eq!(nmea.fields[1], "T");
        assert_eq!(nmea.fields[2], "000.11");
        assert_eq!(nmea.fields[3], "N");
        assert_eq!(nmea.fields[4], "A");

        let input = "$WIXDR,C,+023.1,C,TEMP,P,0.9989,B,PRESS,H,040,P,RH*25";
        let nmea = NmeaParser::to_nmea(input);
        assert_eq!(nmea.talker_id, "WI");
        assert_eq!(nmea.message_id, "XDR");
        assert_eq!(nmea.fields[0], "C");
        assert_eq!(nmea.fields[1], "+023.1");
        assert_eq!(nmea.fields[2], "C");
        assert_eq!(nmea.fields[3], "TEMP");
        assert_eq!(nmea.fields[4], "P");
        assert_eq!(nmea.fields[5], "0.9989");
        assert_eq!(nmea.fields[6], "B");
        assert_eq!(nmea.fields[7], "PRESS");
        assert_eq!(nmea.fields[8], "H");
        assert_eq!(nmea.fields[9], "040");
        assert_eq!(nmea.fields[10], "P");
        assert_eq!(nmea.fields[11], "RH");

        let input = "$GPGGA,113342.000,5045.7837,N,00132.4127,W,1,06,1.3,-10.2,M,47.8,M,,0000*56";
        let nmea = NmeaParser::to_nmea(input);
        assert_eq!(nmea.talker_id, "GP");
        assert_eq!(nmea.message_id, "GGA");
        assert_eq!(nmea.fields[0], "113342.000");
        assert_eq!(nmea.fields[1], "5045.7837");
        assert_eq!(nmea.fields[2], "N");
        assert_eq!(nmea.fields[3], "00132.4127");
        assert_eq!(nmea.fields[4], "W");
        assert_eq!(nmea.fields[5], "1");
        assert_eq!(nmea.fields[6], "06");
        assert_eq!(nmea.fields[7], "1.3");
        assert_eq!(nmea.fields[8], "-10.2");
        assert_eq!(nmea.fields[9], "M");
        assert_eq!(nmea.fields[10], "47.8");
        assert_eq!(nmea.fields[11], "M");

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
