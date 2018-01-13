use std::num::ParseIntError;
use std::num::ParseFloatError;

#[derive(Debug, Fail)]
pub enum ParseNumberError {
    #[fail(display = "{}", _0)]
    ParseIntError(ParseIntError),
    #[fail(display = "{}", _0)]
    ParseFloatError(ParseFloatError),
}

impl From<ParseFloatError> for ParseNumberError {
    fn from(error: ParseFloatError) -> Self {
        ParseNumberError::ParseFloatError(error)
    }
}

impl From<ParseIntError> for ParseNumberError {
    fn from(error: ParseIntError) -> Self {
        ParseNumberError::ParseIntError(error)
    }
}
