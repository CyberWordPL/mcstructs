use std::io::{BufReader, Read};

use thiserror::Error;

use byteorder::ReadBytesExt;

#[derive(Error, Debug, PartialEq)]
pub enum VarIntReadError
{
    #[error("Error reading from Buffer")]
    ReadError,
    #[error("VarInt you reading is too long")]
    VarIntTooLong
}

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

/// Read VarInt from BufReader
pub fn read_varint_bufreader<R: ?Sized>(mut reader: BufReader<&[u8]>) -> Result<i32, VarIntReadError>
{
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
pub fn read_varlong_bufreader<R: ?Sized + Read>(reader: &mut BufReader<R>) -> Result<i64, VarIntReadError>
{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_varint_bufreader_1() {
        let bytes: &[u8; 2] = &[0xFF,0x01];
        let reader: BufReader<&[u8]> = BufReader::new(bytes);

        let result: Result<i32, VarIntReadError> = read_varint_bufreader::<&[u8]>(reader);

        assert_eq!(result, Ok(0xFF));
    }
}