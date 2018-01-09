//! This crate provides char, str and string wrappers which
//! have an "is-ascii" soft constraint.
//!
//! As it is a soft constraint it can be violated, while a violation
//! is (normally) a bug it _does not_ introduce any safety issues.
//! In this soft-ascii-string differs to e.g. [ascii](https://crates.io/crates/ascii)
//! which uses a hard constraint and where a violation does brake
//! rust safety and potentially introduces undefined behaviour.
//!
//! Soft-ascii-string is suited for situations where many places
//! (e.g. external libraries) output strings which should be
//! ascii and which you do not want to iterate over to assure
//! they are ascii but where you neither want to use a unsafe
//! conversions as it would be required by the ascii crate.
//!
//! This crate is not necessarily suited if you want to rally on the string
//! being ascii on a safety level, you might want to consider using
//! [ascii](https://crates.io/crates/ascii) in that case.


pub use self::soft_char::*;
pub use self::soft_str::*;
pub use self::soft_string::*;

#[macro_use]
mod macros;

pub mod error;
mod soft_char;
// note while they are separated for readability str/string
// still do form one unit, i.e. there is a cyclic reference
// between SoftAsciiString<->SoftAsciiStr and others
mod soft_str;
mod soft_string;


// - IndexMut (returns a &mut SoftAsciiStr)
// - DerefMut (returns a &mut SoftAsciiStr)
// - fuzzed test for forwarded method
