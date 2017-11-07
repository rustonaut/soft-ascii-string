use std::fmt;
use std::error::Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct StringFromStrError;
impl Error for StringFromStrError {
    fn description(&self) -> &str {
        "&str does contain non us-ascii chars and can not be converted to a SoftAsciiString"
    }
}

impl fmt::Display for StringFromStrError {
    fn fmt(&self, fter: &mut fmt::Formatter) -> fmt::Result {
        write!(fter, "{}", self.description())
    }
}


