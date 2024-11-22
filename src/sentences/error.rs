use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertNMEA0183Error {
    #[error("Failed to parse field")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Failed to parse field")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Cant convert to enum, field did not match any enumerators: {0}")]
    ConvertToEnumError(String),
}


