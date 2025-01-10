use std::io;

use bincode::{
    config::{self, Configuration},
    error::{DecodeError, EncodeError},
    Decode, Encode,
};

static BINCODE_CONFIG: Configuration = config::standard();

pub fn decode_slice<D: Decode>(bytes: &[u8]) -> Result<D, DecodeError> {
    bincode::decode_from_slice(bytes, BINCODE_CONFIG).map(|(value, _)| value)
}

pub fn encode<E: Encode, W: io::Write>(value: E, writer: &mut W) -> Result<usize, EncodeError> {
    bincode::encode_into_std_write(value, writer, BINCODE_CONFIG)
}
