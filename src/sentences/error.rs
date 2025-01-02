use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseNMEA0183Error {
    #[error("Failed to parse field")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Failed to parse field")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Cant convert to enum, field did not match any enumerators: {0}")]
    ConvertToEnumError(String),
    #[error("Cant parse NMEA0183 sentence from PEG grammar.")]
    ParseGrammarError,
    #[error("NMEA0183 checksum verification failed.")]
    NMEA0183ChecksumError,
}


