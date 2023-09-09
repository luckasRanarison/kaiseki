use bincode::error::{DecodeError, EncodeError};
use std::{fmt, io, num::ParseIntError};

#[derive(Debug, Clone)]
pub enum Error {
    IOError(io::Error),
    EncodeError(EncodeError),
    DecodeError(DecodeError),
    ParseIntError(ParseIntError),
    FstBuilderError(fst::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<EncodeError> for Error {
    fn from(value: EncodeError) -> Self {
        Self::EncodeError(value)
    }
}

impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<fst::Error> for Error {
    fn from(value: fst::Error) -> Self {
        Self::FstBuilderError(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(value) => value.fmt(f),
            Error::EncodeError(value) => value.fmt(f),
            Error::DecodeError(value) => value.fmt(f),
            Error::ParseIntError(value) => value.fmt(f),
            Error::FstBuilderError(value) => value.fmt(f),
        }
    }
}
