
# soft-ascii-string &emsp; [![Build Status](https://travis-ci.org/1aim/soft_ascii_string.svg?branch=master)](https://travis-ci.org/1aim/soft_ascii_string)

**char/str/string wrappers which add a "is-ascii" soft constraint**

---

soft-ascii-string provides char, str and string wrapper which
add an "is-ascii" soft constraint.

As it is a soft constraint it can be violated, while a violation
is (normally) a bug it _does not_ introduce any safety issues.
In this soft-ascii-string differs to e.g. [ascii](https://crates.io/crates/ascii)
which uses a hard constraint and where a violation does brake
rust safety and potentially introduces undefined behaviour.

Soft-ascii-string is suited for situations where many places
(e.g. external libraries) output strings which should be
ascii and which you do not want to iterate over to assure
they are ascii but where you neither want to use a unsafe
conversions as it would be required by the ascii crate.

This crate is not necessarily suited if you want to rally on the string
being ascii on a safety level, you might want to consider using
[ascii](https://crates.io/crates/ascii) in that case.

Documentation can be [viewed on docs.rs](https://docs.rs/soft-ascii-string).

## Example

```rust
extern crate soft_ascii_string;

use soft_ascii_string::{SoftAsciiChar, SoftAsciiStr, SoftAsciiString};

fn main() {
    // encoder_stub should encode all non-ascii chars
    // but it's a complex external dependency so we do
    // not want to rely on it on a safety level
    let mut ascii = SoftAsciiString::from_string_unchecked(external::encoder_stub("magic↓"));

    // we know ":" is ascii so no unnecessary checks here
    ascii.push(SoftAsciiChar::from_char_unchecked(':'));
    // we know "abcde" is ascii so no unnecessary checks here
    ascii.push_str(SoftAsciiStr::from_str_unchecked("abcde"));

    // lets assume we got this from somewhere
    let other_input = "other string";
    let part = SoftAsciiStr::from_str(other_input).expect("other_input should have been ascii");
    ascii.push_str(part);

    let mut clone = SoftAsciiString::with_capacity(ascii.len());
    // the chars(), char_indices() operators return a
    // iterator returning SoftAsciiChars
    for ch in ascii.chars() {
        clone.push(ch);
    }

    // we do all kind of cost transformations
    // without having to revalidate that it is
    // ascii as long as we do not want to rely on it
    internal::costy_transformations(&mut ascii);

    // when running unsafe code we really do not want a bug
    // which introduced non ascii code to introduce unsafety
    // so we can just validate if it really is ascii.
    // On the other hand as long as we do not need a 100% guarantee
    // for security reason we do not need to call revalidate.
    match ascii.revalidate_soft_constraint() {
        Ok(ascii) => {
            unsafe {external::requires_ascii(ascii.as_bytes())}
        },
        Err(err) => panic!("non-ascii content in ascii string")
    }

}


mod internal {
    use soft_ascii_string::SoftAsciiString;

    pub fn costy_transformations(s: &mut SoftAsciiString) {
        let s2 = s.clone();
        s.insert_str(0, &*s2)
    }
}

mod external {

    // lets assume this is an external function only working with ascii
    pub unsafe fn requires_ascii(b: &[u8])  {}

    // lets assume this is more complex and
    // from a external dependency, we assume
    // it returns ascii, but what if there is
    // a bug
    pub fn encoder_stub(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        for ch in s.chars() {
            if ' ' <= ch && ch <= '~' {
                out.push(ch)
            } else { out.push('?') }
        }
        out
    }

}
```

Error handling:

```rust
extern crate soft_ascii_string;

use soft_ascii_string::{SoftAsciiChar, SoftAsciiStr, SoftAsciiString};

fn main() {
    let non_ascii_input: String = "←↓↓↓".into();
    match SoftAsciiString::from_string(non_ascii_input) {
        Ok(okvalue) => panic!("the string should not have been ascii"),
        Err(err) => {
            let original_source: String = err.into_source();
            println!("the input was: {:?}", original_source)
        }
    }
}
```




## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
