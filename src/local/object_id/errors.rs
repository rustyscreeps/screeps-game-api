use std::{error::Error, fmt, num::ParseIntError};

#[derive(Debug, Clone)]
pub enum RawObjectIdParseError {
    Parse(ParseIntError),
    LargeValue(u128),
}

impl fmt::Display for RawObjectIdParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RawObjectIdParseError::Parse(e) => {
                write!(f, "error parsing object id hex digits: {e}")
            }
            RawObjectIdParseError::LargeValue(value) => write!(
                f,
                "string contained hex value too big be object id. \
                 value {value} bigger than maximum for 24 digits"
            ),
        }
    }
}

impl Error for RawObjectIdParseError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            RawObjectIdParseError::Parse(e) => Some(e),
            RawObjectIdParseError::LargeValue(_) => None,
        }
    }
}

impl From<ParseIntError> for RawObjectIdParseError {
    fn from(e: ParseIntError) -> Self {
        RawObjectIdParseError::Parse(e)
    }
}

impl RawObjectIdParseError {
    pub(crate) const fn value_too_large(val: u128) -> Self {
        RawObjectIdParseError::LargeValue(val)
    }
}
