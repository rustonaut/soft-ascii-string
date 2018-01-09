use std::cmp::PartialEq;
use std::fmt::{self, Display};
use std::str;
use std::char::{
    self, 
    ToUppercase, ToLowercase, 
    EscapeDebug, EscapeDefault, EscapeUnicode
};
#[allow(unused_imports)]
use std::ascii::AsciiExt;

use error::FromSourceError;


///a `char` wrapper with a "is us-ascii" soft constraint
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, 
    PartialOrd, Ord, Hash
)]
pub struct SoftAsciiChar(char);


impl SoftAsciiChar {

    #[inline(always)]
    pub fn from_char_unchecked(ch: char) -> Self {
        SoftAsciiChar(ch)
    }

    pub fn from_char(ch: char) -> Result<Self, FromSourceError<char>> {
        if ch.is_ascii() {
            Ok(SoftAsciiChar(ch))
        } else {
            Err(FromSourceError::new(ch))
        }
    }

    #[inline]
    pub fn is_ascii(self) -> bool {
        self.0.is_ascii()
    }

    pub fn revalidate_soft_constraint(self) -> Result<Self, char> {
        if self.is_ascii() {
            Ok(self)
        } else {
            Err(self.0)
        }
    }
}

impl Display for SoftAsciiChar {
    fn fmt(&self, fter: &mut fmt::Formatter) -> fmt::Result {
        let ch: char = (*self).into();
        ch.fmt(fter)
    }
}

impl Into<char> for SoftAsciiChar {
    fn into(self) -> char {
        self.0
    }
}

//Deref does not work as all `&self`-logic methods use `self` because Self: Copy
macro_rules! impl_wrapping {
    (pub > $(fn $name:ident(self$(, $param:ident: $tp:ty)*) -> $ret:ty),*) => (
        impl SoftAsciiChar {$(
            #[inline]
            pub fn $name(self $(, $param: $tp)*) -> $ret {
                char::$name(self.0 $(, $param)*)
            }
        )*}
    );
}

impl_wrapping! {
    pub >
    fn is_digit(self, radix: u32) -> bool,
    fn to_digit(self, radix: u32) -> Option<u32>,
    fn escape_unicode(self) -> EscapeUnicode,
    fn escape_debug(self) -> EscapeDebug,
    fn escape_default(self) -> EscapeDefault,
    fn len_utf8(self) -> usize,
    fn len_utf16(self) -> usize,
    fn encode_utf8(self, dst: &mut [u8]) -> &mut str,
    fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16],
    fn is_alphabetic(self) -> bool,
    fn is_lowercase(self) -> bool,
    fn is_uppercase(self) -> bool,
    fn is_whitespace(self) -> bool,
    fn is_alphanumeric(self) -> bool,
    fn is_control(self) -> bool,
    fn is_numeric(self) -> bool,
    fn to_lowercase(self) -> ToLowercase,
    fn to_uppercase(self) -> ToUppercase
}

impl PartialEq<char> for SoftAsciiChar {
    fn eq(&self, other: &char) -> bool {
        self.0 == *other
    }
}

impl PartialEq<SoftAsciiChar> for char {
    fn eq(&self, other: &SoftAsciiChar) -> bool {
        *self == other.0
    }
}

//TODO FromStr with custom error

#[cfg(test)]
mod test {

    mod SoftAsciiChar {
        #![allow(non_snake_case)]
        use super::super::SoftAsciiChar;
        use error::FromSourceError;

        #[test]
        fn from_char() {
            let res: SoftAsciiChar = assert_ok!(SoftAsciiChar::from_char('a'));
            assert_eq!(res, 'a');
            let res = assert_err!(SoftAsciiChar::from_char('↓'));
            assert_eq!(res, FromSourceError::new('↓'));
        }

        #[test]
        fn from_char_unchecked() {
            let a: SoftAsciiChar = SoftAsciiChar::from_char_unchecked('a');
            assert_eq!(a, 'a');
            let a: SoftAsciiChar = SoftAsciiChar::from_char_unchecked('↓');
            assert_eq!(a, '↓');
        }

        #[test]
        fn revalidate_soft_constraint() {
            let a: SoftAsciiChar = SoftAsciiChar::from_char_unchecked('a');
            let val = assert_ok!(a.revalidate_soft_constraint());
            assert_eq!(val, 'a');
            let a: SoftAsciiChar = SoftAsciiChar::from_char_unchecked('↓');
            let val = assert_err!(a.revalidate_soft_constraint());
            assert_eq!(val, '↓');
        }
    }
}