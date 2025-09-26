pub mod sentences;
use crate::sentences::seaview::{
    psvdy::Svdy, psvsd::Svsd, psvsh::Svsh, psvsi::Svsi, psvss::Svss, psvst::Svst, psvsv::Svsv,
    psvsw::Svsw,
};
use pest::Parser;
use pest_derive::Parser;
use sentences::{
    error::ParseNMEA0183Error, gga::Gga, hdm::Hdm, hdt::Hdt, mwv::Mwv, pgilt::Gilt, xdr::Xdr,
};

#[derive(Debug)]
pub enum Sentence {
    Unknown,
    Mwv(sentences::mwv::Mwv),
    Xdr(sentences::xdr::Xdr),
    Gga(sentences::gga::Gga),
    Hdm(sentences::hdm::Hdm),
    Hdt(sentences::hdt::Hdt),
    Gilt(sentences::pgilt::Gilt),
    Svdy(sentences::seaview::psvdy::Svdy),
    Svss(sentences::seaview::psvss::Svss),
    Svsd(sentences::seaview::psvsd::Svsd),
    Svst(sentences::seaview::psvst::Svst),
    Svsi(sentences::seaview::psvsi::Svsi),
    Svsv(sentences::seaview::psvsv::Svsv),
    Svsh(sentences::seaview::psvsh::Svsh),
    Svsw(sentences::seaview::psvsw::Svsw),
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
    pub fn parse(nmea_sentence: &str) -> Result<Sentence, ParseNMEA0183Error> {
        let nmea = NmeaParser::to_nmea(nmea_sentence.trim())?;
        Ok(match nmea.message_id.as_ref() {
            "MWV" => Sentence::Mwv(Mwv::try_from(nmea)?),
            "GGA" => Sentence::Gga(Gga::try_from(nmea)?),
            "XDR" => Sentence::Xdr(Xdr::try_from(nmea)?),
            "HDM" => Sentence::Hdm(Hdm::try_from(nmea)?),
            "HDT" => Sentence::Hdt(Hdt::try_from(nmea)?),
            "GILT" => Sentence::Gilt(Gilt::try_from(nmea)?),
            "SVDY" => Sentence::Svdy(Svdy::try_from(nmea)?),
            "SVSS" => Sentence::Svss(Svss::try_from(nmea)?),
            "SVSD" => Sentence::Svsd(Svsd::try_from(nmea)?),
            "SVST" => Sentence::Svst(Svst::try_from(nmea)?),
            "SVSI" => Sentence::Svsi(Svsi::try_from(nmea)?),
            "SVSV" => Sentence::Svsv(Svsv::try_from(nmea)?),
            "SVSH" => Sentence::Svsh(Svsh::try_from(nmea)?),
            "SVSW" => Sentence::Svsw(Svsw::try_from(nmea)?),
            _ => Sentence::Unknown,
        })
    }

    fn verify_checksum(sentence: &str, checksum: String) -> Result<(), ParseNMEA0183Error> {
        let sentence = &sentence[1..sentence.len() - 3];
        let checksum = u8::from_str_radix(&checksum[1..checksum.len()], 16)?;
        let mut temp: u8 = 0;
        for b in sentence.as_bytes() {
            temp ^= b;
        }

        if checksum != temp {
            return Err(sentences::error::ParseNMEA0183Error::NMEA0183ChecksumError);
        }

        Ok(())
    }

    fn to_nmea(nmea_sentence: &str) -> Result<Nmea, ParseNMEA0183Error> {
        let parsed = match NmeaPest::parse(Rule::NMEA, nmea_sentence) {
            Ok(p) => p,
            Err(_) => return Err(ParseNMEA0183Error::ParseGrammarError),
        }
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
                Rule::checksum => checksum = Some(pair.as_str().to_string()),
                _ => {}
            }
        }

        //Return an error if the checksum is invalid
        if checksum.is_some() {
            NmeaParser::verify_checksum(nmea_sentence, checksum.unwrap())?;
        }

        Ok(Nmea {
            talker_id,
            message_id: message_type,
            fields,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use sentences::{gga::FixQuality, pgilt::ZOrientation, TransducerReading, UnitsOfMeasurement};
    use uom::si::{length::meter, time::second};

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
                if let Some(TransducerReading::Temperature(r)) = &nmea.readings[0] {
                    assert_eq!(r.reading, Some(23.1));
                    assert_eq!(r.units, Some(UnitsOfMeasurement::Celsius));
                    assert_eq!(r.name, Some("TEMP".into()));
                }
                if let Some(TransducerReading::Pressure(r)) = &nmea.readings[1] {
                    assert_eq!(r.reading, Some(0.9989));
                    assert_eq!(r.units, Some(UnitsOfMeasurement::Bar));
                    assert_eq!(r.name, Some("PRESS".into()));
                }
                if let Some(TransducerReading::Pressure(r)) = &nmea.readings[2] {
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
        let input = "$PGILT,A,+00,D,+01,D,+1,TILT*35";
        let output = NmeaParser::parse(input).unwrap();
        println!("{:?}", output);
        match output {
            Sentence::Gilt(nmea) => {
                assert_eq!(nmea.talker_id, "P");
                assert_eq!(nmea.message_id, "GILT");
                if let Some(TransducerReading::AngularDisplacement(r)) = nmea.x_tilt {
                    assert_eq!(r.reading, Some(0.0));
                    assert_eq!(r.units, Some(UnitsOfMeasurement::Degrees));
                    assert_eq!(r.name, None);
                }
                if let Some(TransducerReading::AngularDisplacement(r)) = nmea.y_tilt {
                    assert_eq!(r.reading, Some(1.0));
                    assert_eq!(r.units, Some(UnitsOfMeasurement::Degrees));
                    assert_eq!(r.name, None);
                }
                assert_eq!(nmea.z_orientation, Some(ZOrientation::FacingUpright));
                assert_eq!(nmea.sensor_name, Some("TILT".to_string()));
            }
            _ => panic!("Expected Gga"),
        }
    }

    #[test]
    #[ignore]
    fn test_svsd_all_zero() {
        use approx::assert_abs_diff_eq;

        let input = "$PSVSD,128.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.000 0,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000*49";
        let output = NmeaParser::parse(input).unwrap();

        match output {
            Sentence::Svsd(nmea) => {
                let eps = 1e-6;

                assert_eq!(nmea.talker_id, "P");
                assert_eq!(nmea.message_id, "SVSD");

                // There should be 14 bins
                assert_eq!(nmea.directional_spectrum_bins.len(), 14);

                for e in nmea.directional_spectrum_bins.iter() {
                    assert_abs_diff_eq!(*e, 0.0f32, epsilon = eps);
                }
            }
            _ => panic!("Expected Svss"),
        }
    }

    #[test]
    fn test_svsv_parse() {
        let input = "$PSVSV,0.000,0.024,0.037,-0.663*7C";
        let output = NmeaParser::parse(input).unwrap();

        match output {
            Sentence::Svsv(nmea) => {
                assert_eq!(nmea.talker_id, "P");
                assert_eq!(nmea.message_id, "SVSV");

                // Check numeric fields
                assert_eq!(nmea.time.unwrap().get::<second>(), 0.000);
                assert_eq!(nmea.north.unwrap().get::<meter>(), 0.024);
                assert_eq!(nmea.east.unwrap().get::<meter>(), 0.037);
                assert_eq!(nmea.up.unwrap().get::<meter>(), -0.663);
            }
            _ => panic!("Expected Svsv"),
        }
    }

    #[test]
    fn test_svsi_identity_parse() {
        let input = "$PSVSI,WINDMILL*73";
        let output = NmeaParser::parse(input).unwrap();

        match output {
            Sentence::Svsi(nmea) => {
                assert_eq!(nmea.talker_id, "P");
                assert_eq!(nmea.message_id, "SVSI");

                let expected = "WINDMILL";
                assert_eq!(nmea.identity, expected);
            }
            _ => panic!("Expected Svst"),
        }
    }

    #[test]
    fn test_svst_timestamp_parse() {
        let input = "$PSVST,2020-10-02 16:04:53*58";
        let output = NmeaParser::parse(input).unwrap();

        match output {
            Sentence::Svst(nmea) => {
                assert_eq!(nmea.talker_id, "P");
                assert_eq!(nmea.message_id, "SVST");

                let expected =
                    NaiveDateTime::parse_from_str("2020-10-02 16:04:53", "%Y-%m-%d %H:%M:%S")
                        .unwrap();
                assert_eq!(nmea.timestamp, Some(expected));
            }
            _ => panic!("Expected Svst"),
        }
    }

    #[test]
    fn test_svss_all_zero() {
        use approx::assert_abs_diff_eq;

        let input = "$PSVSS,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000*55";
        let output = NmeaParser::parse(input).unwrap();

        match output {
            Sentence::Svss(nmea) => {
                let eps = 1e-6;

                assert_eq!(nmea.talker_id, "P");
                assert_eq!(nmea.message_id, "SVSS");

                let bin = &nmea.bin;

                assert_abs_diff_eq!(bin.period, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.es_mag, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.n_re, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.n_im, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.e_re, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.e_im, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.u_re, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.u_im, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.u_mag, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.theta, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.a1, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.b1, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.a2, 0.0f32, epsilon = eps);
                assert_abs_diff_eq!(bin.b2, 0.0f32, epsilon = eps);
            }
            _ => panic!("Expected Svss"),
        }
    }

    #[test]
    fn test_svdy() {
        use approx::assert_abs_diff_eq;

        let input = "$PSVDY,-0.210,-0.116,9.825,-0.0044,0.0011,-0.0044,217.3,0.6,-1.4,0.021,-0.012,9.828,703*6C";
        let output = NmeaParser::parse(input).unwrap();

        match output {
            Sentence::Svdy(nmea) => {
                let eps = 1e-4; // epsilon for small float comparison

                assert_eq!(nmea.talker_id, "P");
                assert_eq!(nmea.message_id, "SVDY");

                assert_abs_diff_eq!(nmea.acc_x.unwrap().value, -0.210, epsilon = eps);
                assert_abs_diff_eq!(nmea.acc_y.unwrap().value, -0.116, epsilon = eps);
                assert_abs_diff_eq!(nmea.acc_z.unwrap().value, 9.825, epsilon = eps);

                // assert_abs_diff_eq!(nmea.gyr_p.unwrap().value, -0.0044, epsilon = eps);
                // assert_abs_diff_eq!(nmea.gyr_q.unwrap().value, 0.0011, epsilon = eps);
                // assert_abs_diff_eq!(nmea.gyr_r.unwrap().value, -0.0044, epsilon = eps);

                // assert_abs_diff_eq!(nmea.heading.unwrap().value, 217.3, epsilon = 1e-3);
                // assert_abs_diff_eq!(nmea.pitch.unwrap().value, 0.6, epsilon = eps);
                // assert_abs_diff_eq!(nmea.roll.unwrap().value, -1.4, epsilon = eps);

                assert_abs_diff_eq!(nmea.acc_n.unwrap().value, 0.021, epsilon = eps);
                assert_abs_diff_eq!(nmea.acc_e.unwrap().value, -0.012, epsilon = eps);
                assert_abs_diff_eq!(nmea.acc_u.unwrap().value, 9.828, epsilon = eps);

                assert_eq!(nmea.index, Some(703));
            }
            _ => panic!("Expected Svdy"),
        }
    }

    #[test]
    fn test_checksum() {
        let input = "$WIMWV,049,R,000.03,N,A*03";
        let _ = NmeaParser::parse(input).unwrap();

        let input = "$WIMWV,180,T,000.11,N,A*02";
        let _ = NmeaParser::parse(input).unwrap();

        let input = "$PGILT,A,+00,D,+01,D,+1,TILT*35";
        let _ = NmeaParser::parse(input).unwrap();

        let input = "$GPGGA,113342.000,5045.7837,N,00132.4127,W,1,06,1.3,-10.2,M,47.8,M,,0000*56";
        let _ = NmeaParser::parse(input).unwrap();

        let input = "$GPGSV,2,2,08,15,30,050,47,19,09,158,,26,12,281,40,27,38,173,41*7B";
        let _ = NmeaParser::parse(input).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_checksum_fail() {
        let input = "$GPGGA,113342.000,5045.7837,N,00132.4127,W,1,06,1.3,-10.2,M,47.8,M,,0000*5F";
        let _ = NmeaParser::parse(input).unwrap();
    }
}
