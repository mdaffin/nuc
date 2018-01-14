#[macro_use]
extern crate failure;

use std::fmt;
use std::str::FromStr;

mod error;
pub use error::ParseNumberError;

pub enum Number {
    U4(u8),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl Number {
    pub fn fmt_signed(&self) -> String {
        use Number::*;
        match *self {
            U4(i) => format!("{}", i as i8),
            U8(i) => format!("{}", i as i8),
            U16(i) => format!("{}", i as i16),
            U32(i) => format!("{}", i as i32),
            U64(i) => format!("{}", i as i64),
        }
    }
}

impl FromStr for Number {
    type Err = ParseNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("_", "");
        Ok(match &s {
            s if s.starts_with("0x") => {
                match s.len() {
                    0...3 => Number::U4(u8::from_str_radix(&s[2..], 16)?),
                    3...4 => Number::U8(u8::from_str_radix(&s[2..], 16)?),
                    5...6 => Number::U16(u16::from_str_radix(&s[2..], 16)?),
                    6...10 => Number::U32(u32::from_str_radix(&s[2..], 16)?),
                    _ => Number::U64(u64::from_str_radix(&s[2..], 16)?),
                }
            }
            s if s.starts_with("0b") => {
                match s.len() {
                    0...6 => Number::U4(u8::from_str_radix(&s[2..], 2)?),
                    7...10 => Number::U8(u8::from_str_radix(&s[2..], 2)?),
                    11...18 => Number::U16(u16::from_str_radix(&s[2..], 2)?),
                    19...34 => Number::U32(u32::from_str_radix(&s[2..], 2)?),
                    _ => Number::U64(u64::from_str_radix(&s[2..], 2)?),
                }
            }
            s if s.starts_with("-") => Number::U64(s.parse::<i64>()? as u64),
            s => Number::U64(s.parse()?),
        })
    }
}


impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Number::*;
        match *self {
            U4(i) => f.pad(&format!("{}", i)),
            U8(i) => f.pad(&format!("{}", i)),
            U16(i) => f.pad(&format!("{}", i)),
            U32(i) => f.pad(&format!("{}", i)),
            U64(i) => f.pad(&format!("{}", i)),
        }
    }
}

impl fmt::Binary for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Number::*;
        match *self {
            U4(i) => f.pad(&format!("{:#006b}", i)),
            U8(i) => f.pad(&format!("{:#010b}", i)),
            U16(i) => f.pad(&format!("{:#018b}", i)),
            U32(i) => f.pad(&format!("{:#034b}", i)),
            U64(i) => f.pad(&format!("{:#066b}", i)),
        }
    }
}


impl fmt::UpperHex for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Number::*;
        match *self {
            U4(i) => f.pad(&format!("{:#03X}", i)),
            U8(i) => f.pad(&format!("{:#04X}", i)),
            U16(i) => f.pad(&format!("{:#06X}", i)),
            U32(i) => f.pad(&format!("{:#010X}", i)),
            U64(i) => f.pad(&format!("{:#018X}", i)),
        }
    }
}

impl fmt::LowerHex for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Number::*;
        match *self {
            U4(i) => f.pad(&format!("{:#03x}", i)),
            U8(i) => f.pad(&format!("{:#04x}", i)),
            U16(i) => f.pad(&format!("{:#06x}", i)),
            U32(i) => f.pad(&format!("{:#010x}", i)),
            U64(i) => f.pad(&format!("{:#018x}", i)),
        }
    }
}
