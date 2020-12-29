# CO₂: Human ciphers, implemented in Rust.

CO₂ (a.k.a. "Cipherium Dioxide", or perhaps "Cipher Dioxide", or just "C-O-2" if you insist) is an extensible Rust library for "human ciphers" (which are simpler ciphers, such as the Caesar cipher or Vigenère cipher).

## Example (proposed API)
```rs
use co2::prelude::*;
use subst::VigenereBuilder;

fn main() {
    // Build the cipher encoder/decoder for a standard Vigenère cipher
    // with the key `ciphersarecool`. Everything else should be set to
    // the default.
    let c = VigenereBuilder::standard().key("ciphersarecool").build();
    // Encode "Hello, world!" using the Vigenère cipher with the
    // previously set key. This returns a `String`, so to compare
    // it with a `&str`, it needs some coaxing.
    assert_eq!(&c.encode("Hello, world!"), "Jmass, ngrch!");
    // Decode some encoded text with the previously set key.
    // This returns a `String` too.
    assert_eq!(&c.decode("Kb lvvbk!"), "It works!");
}
```