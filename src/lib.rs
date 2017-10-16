pub use self::soft_char::*;
pub use self::soft_str::*;
pub use self::soft_string::*;

#[macro_use]
mod macros;

mod soft_char;
//note while they are seperated for redabilitye str/string
// still do form one unit, i.e. there is a cyclic reference
// between them str->ToOwned, string->Deref/etc.
mod soft_str;
mod soft_string;

//TODO:
// - FromStr with custom error
// - IndexMut (returns a &mut SoftAsciiStr)
// - DerefMut (returns a &mut SoftAsciiStr)
// - fuzzed test for forwarded method


