#[macro_use]
extern crate failure;

use std::fmt;
use std::str::FromStr;

mod error;
pub use error::ParseNumberError;

pub enum Number {
    U8(u8),
    U16(u16),
    U32(u32),
    I8(i8),
    I16(i16),
    I32(i32),
}

impl FromStr for Number {
    type Err = ParseNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Number::I8(s.parse()?))
    }
}


impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Number::*;
        match *self {
            U8(i) => write!(f, "{}", i),
            U16(i) => write!(f, "{}", i),
            U32(i) => write!(f, "{}", i),
            I8(i) => write!(f, "{}", i),
            I16(i) => write!(f, "{}", i),
            I32(i) => write!(f, "{}", i),
        }
    }
}

impl fmt::Binary for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Number::*;
        match *self {
            U8(i) => write!(f, "{:#010b}", i),
            U16(i) => write!(f, "{:#018b}", i),
            U32(i) => write!(f, "{:#034b}", i),
            I8(i) => write!(f, "{:#010b}", i),
            I16(i) => write!(f, "{:#018b}", i),
            I32(i) => write!(f, "{:#034b}", i),
        }
    }
}


impl fmt::UpperHex for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Number::*;
        match *self {
            U8(i) => write!(f, "{:#04X}", i),
            U16(i) => write!(f, "{:#06X}", i),
            U32(i) => write!(f, "{:#010X}", i),
            I8(i) => write!(f, "{:#04X}", i),
            I16(i) => write!(f, "{:#06X}", i),
            I32(i) => write!(f, "{:#010X}", i),
        }
    }
}

impl fmt::LowerHex for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Number::*;
        match *self {
            U8(i) => write!(f, "0x{:04x}", i),
            U16(i) => write!(f, "0x{:06x}", i),
            U32(i) => write!(f, "0x{:010x}", i),
            I8(i) => write!(f, "0x{:04x}", i),
            I16(i) => write!(f, "0x{:06x}", i),
            I32(i) => write!(f, "0x{:010x}", i),
        }
    }
}
