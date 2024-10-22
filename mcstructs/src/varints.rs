use std::io::{BufReader, Read};

use thiserror::Error;

use byteorder::ReadBytesExt;

#[derive(Error, Debug)]
pub enum VarIntReadError
{
    #[error("Error reading from Buffer")]
    ReadError,
    #[error("VarInt you reading is too long")]
    VarIntTooLong
}

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub fn read_varint<R: ?Sized>(reader: &mut BufReader<R>) -> Result<i32, VarIntReadError>
{
    let mut value: i32 = 0;
    let mut position: u8 = 0;


    loop {

        let current_byte: u8 = match reader.read_u8()
        {
            Ok(byte) => byte,
            Err(_) => return Err(VarIntReadError::VarIntTooLong)
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

pub fn read_varlong<R: ?Sized>(reader: &mut BufReader<R>) -> Result<i32, VarIntReadError>
{
    let mut value: i32 = 0;
    let mut position: u8 = 0;


    loop {

        let current_byte: u8 = match reader.read_u8()
        {
            Ok(byte) => byte,
            Err(_) => return Err(VarIntReadError::VarIntTooLong)
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