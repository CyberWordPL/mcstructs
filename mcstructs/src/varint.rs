use std::io::{BufReader, BufWriter, Read, Write};

use thiserror::Error;

use byteorder::{ReadBytesExt, WriteBytesExt};
use crate::utils::{bitshift_i32_with_sign, bitshift_i64_with_sign};

#[derive(Error, Debug, PartialEq)]
pub enum VarIntReadError
{
    #[error("Error reading from Buffer")]
    ReadError,
    #[error("VarInt you reading is too long")]
    VarIntTooLong
}

#[derive(Error, Debug, PartialEq)]
pub enum VarIntWriteError
{
    #[error("Error writting to Buffer")]
    WriteError,
}

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

/// Read VarInt from BufReader
pub fn read_varint_bufreader<R: Read>(mut reader: BufReader<R>) -> Result<i32, VarIntReadError>  {
    let mut value: i32 = 0;
    let mut position: u8 = 0;


    loop {

        let current_byte: u8 = match reader.read_u8()
        {
            Ok(byte) => byte,
            Err(_) => return Err(VarIntReadError::ReadError)
        };

        value |= ((current_byte & SEGMENT_BITS) as i32) << position;
        position += 7;

        if current_byte & CONTINUE_BIT == 0 {
            break;
        }

        if position >= 32 {
            return Err(VarIntReadError::VarIntTooLong);
        }
    }

    Ok(value)
}

/// Read VarLong from BufReader
pub fn read_varlong_bufreader<R: Read>(mut reader: BufReader<R>) -> Result<i64, VarIntReadError> {
    let mut value: i64 = 0;
    let mut position: u8 = 0;


    loop {

        let current_byte: u8 = match reader.read_u8()
        {
            Ok(byte) => byte,
            Err(_) => return Err(VarIntReadError::ReadError)
        };

        value |= ((current_byte & SEGMENT_BITS) as i64) << position;
        position += 7;

        if current_byte & CONTINUE_BIT == 0 {
            break;
        }

        if position >= 64 {
            return Err(VarIntReadError::VarIntTooLong);
        }
    }

    Ok(value)
}

pub fn write_varint_bufwriter<W: Write>(mut writer: BufWriter<W>, mut value: i32) -> Result<(), VarIntWriteError>  {
    loop {
        if (u8::from(value) & !SEGMENT_BITS) == 0 {
            return match writer.write_u8(u8::from(value)) {
                Ok(()) => Ok(()),
                Err(_) => Err(VarIntWriteError::WriteError)
            }
        }

        match writer.write_u8((u8::from(value) & SEGMENT_BITS) | CONTINUE_BIT) {
            Err(_) => return Err(VarIntWriteError::WriteError),
            _ => {}
        }

        value = bitshift_i32_with_sign(value, 7);
    }
}

pub fn write_varlong_bufwriter<W: Write>(mut writer: BufWriter<W>, mut value: i64) -> Result<(), VarIntWriteError>  {
    loop {
        if (u8::from(value) & !SEGMENT_BITS) == 0 {
            return match writer.write_u8(u8::from(value)) {
                Ok(()) => Ok(()),
                Err(_) => Err(VarIntWriteError::WriteError)
            }
        }

        match writer.write_u8((u8::from(value) & SEGMENT_BITS) | CONTINUE_BIT) {
            Err(_) => return Err(VarIntWriteError::WriteError),
            _ => {}
        }

        value = bitshift_i64_with_sign(value, 7);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_varint_bufreader_1() {
        let bytes: &[u8; 2] = &[0xFF, 0x01];
        let reader: BufReader<&[u8]> = BufReader::new(bytes);

        let result: Result<i32, VarIntReadError> = read_varint_bufreader::<&[u8]>(reader);

        assert_eq!(result, Ok(0xFF));
    }

    #[test]
    fn test_read_varint_bufreader_2() {
        let bytes: &[u8; 6] = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let reader: BufReader<&[u8]> = BufReader::new(bytes);

        let result: Result<i32, VarIntReadError> = read_varint_bufreader::<&[u8]>(reader);

        assert_eq!(result, Err(VarIntReadError::VarIntTooLong));
    }

    fn test_read_varlong_bufreader_1() {
        let bytes: &[u8; 2] = &[0xFF, 0x01];
        let reader: BufReader<&[u8]> = BufReader::new(bytes);

        let result: Result<i32, VarIntReadError> = read_varint_bufreader::<&[u8]>(reader);

        assert_eq!(result, Ok(0xFF));
    }

    #[test]
    fn test_read_varlong_bufreader_2() {
        let bytes: &[u8; 6] = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let reader: BufReader<&[u8]> = BufReader::new(bytes);

        let result: Result<i64, VarIntReadError> = read_varlong_bufreader::<&[u8]>(reader);

        assert_eq!(result, Ok(0x3fbffffffffff));
    }

    #[test]
    fn test_read_varlong_bufreader_2() {
        let bytes: &[u8; 11] = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let reader: BufReader<&[u8]> = BufReader::new(bytes);

        let result: Result<i64, VarIntReadError> = read_varlong_bufreader::<&[u8]>(reader);

        assert_eq!(result, Err(VarIntReadError::VarIntTooLong));
    }
}