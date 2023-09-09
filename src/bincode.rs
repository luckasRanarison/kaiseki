use crate::error::Error;
use bincode::{
    config::{self, Configuration},
    decode_from_slice, encode_into_std_write,
    error::DecodeError,
    Decode, Encode,
};
use lazy_static::lazy_static;
use std::io;

lazy_static! {
    static ref BINCODE_CONFIG: Configuration = config::standard();
}

pub fn decode_slice<D: Decode>(bytes: &[u8]) -> Result<D, Error> {
    let (decoded, written) = decode_from_slice(bytes, *BINCODE_CONFIG)?;
    let additional = bytes.len() - written;

    if additional != 0 {
        return Err(DecodeError::UnexpectedEnd { additional })?;
    }

    Ok(decoded)
}

pub fn encode<E: Encode, W: io::Write>(value: E, writer: &mut W) -> Result<(), Error> {
    encode_into_std_write(value, writer, *BINCODE_CONFIG)?;

    Ok(())
}
