//! Ciphers that are available in CO2.
//!
//! These currently come in two categories:
//!
//! - [`subst`] ([Substitution ciphers]): These ciphers substitute letters. An
//!   example of this would be the [Vigenère cipher] which uses a key and a table
//!   to substitute letters in a piece of text.
//!
//! - [`trans`] ([Transposition ciphers]): These ciphers switch letters around
//!   or shift them. An example of this would be the [Caesar cipher] which simply
//!   shifts the message's alphabet by a certain amount.
//!
//! For the individual ciphers that are available, see the specific category.
//!
//! [Substitution ciphers]: https://en.wikipedia.org/wiki/Substitution_cipher
//! [Transposition ciphers]: https://en.wikipedia.org/wiki/Transposition_cipher
//! [Caesar cipher]: https://en.wikipedia.org/wiki/Caesar_cipher
//! [Vigenère cipher]: https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher

/// TODO
pub mod trans {}
pub mod subst;
