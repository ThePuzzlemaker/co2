//! This module contains various substitution ciphers.
//!
//! Substitution ciphers are a type of cipher that substitutes one alphabet for
//! another.
//!
//! They can come in multiple types, including:
//!
//! - Monoalphabetic ciphers: These ciphers simply substitute one alphabet for
//!   another throughout the whole message. For example, if `a` equals `g` in the
//!   cipher, every `a` in the message is replaced with a `g`.
//!
//! - Polyalphabetic ciphers: These ciphers use more complex logic to substitute
//!   one alphabet for another. This logic sometimes includes a key that
//!   potentially makes the substitution more unique and secure.
//!
//! For more information, perhaps visit [the Wikipedia page on substitution
//! ciphers](https://en.wikipedia.org/wiki/Substitution_cipher)

pub mod mono;
