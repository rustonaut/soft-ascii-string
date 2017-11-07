use std::fmt::{self, Debug};
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


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct FromSourceError<S: Debug+AsRef<str>> {
    pub(crate) source: S
}

impl<S> FromSourceError<S>
    where S: Debug + AsRef<str>
{
    /// returns a reference to the source
    ///
    /// the source is the input which was meant to be converted into a
    /// SoftAsciiStr/String
    pub fn source(&self) -> &S {
        &self.source
    }

    /// returns a str representation of the source
    ///
    /// the source is the input which was meant to be converted into a
    /// SoftAsciiStr/String
    pub fn source_str(&self) -> &str {
        self.source.as_ref()
    }

    // Note that Into, can not be implemented due to possible conflicting
    // implementations
    /// returns the source
    ///
    /// the source is the input which was meant to be converted into a
    /// SoftAsciiStr/String
    pub fn into_source(self) -> S {
        self.source
    }
}



impl<S> Error for FromSourceError<S>
    where S: Debug+AsRef<str>
{
    fn description(&self) -> &str {
        concat!("could not create a SoftAscii representation of the source",
                "as the source contained non us-ascii chars")
    }
}

impl<S> fmt::Display for FromSourceError<S>
    where S: Debug+AsRef<str>
{
    fn fmt(&self, fter: &mut fmt::Formatter) -> fmt::Result {
        write!(fter, "source contains non us-ascii chars: {:?}", self.source.as_ref())
    }
}