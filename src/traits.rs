//! Traits that define how encoding and decoding works.

use crate::error::CO2Result;

/// A trait defining how text is encoded via a cipher.
///
/// # Type Parameters
///
/// - `Pt`: The type used as the plaintext for this cipher. In most cases, this will be a
///   [`String`] or a `&str`.
/// - `Ct`: The type used as the ciphertext for this cipher. In most cases, this will be a
///   [`String`].
pub trait Encoder<Pt, Ct> {
    /// Encode the provided plaintext, returning the ciphertext or an error.
    ///
    /// # Errors
    ///
    /// Arbitrary errors may be returned by this function, depending on the
    /// trait's implementor, via the [`CO2Error`](crate::error::CO2Error) type.
    fn encode(&mut self, plaintext: Pt) -> CO2Result<Ct>;
}

/// A trait defining how text is decoded via a cipher.
///
/// # Type Parameters
///
/// - `Pt`: The type used as the plaintext for this cipher. In most cases, this will be a
///   [`String`].
/// - `Ct`: The type used as the ciphertext for this cipher. In most cases, this will be a
///   [`String`] or a `&str`.
pub trait Decoder<Pt, Ct> {
    /// Decode the provided ciphertext, returning the plaintext or an error.
    ///
    /// # Errors
    ///
    /// Arbitrary errors may be returned by this function, depending on the
    /// trait's implementor, via the [`CO2Error`](crate::error::CO2Error) type.
    fn decode(&mut self, ciphertext: Ct) -> CO2Result<Pt>;
}

/// An extension trait for quick encoding or decoding on types such as `&str`
/// and [`String`].
///
/// This is implemented for every type and is thus sealed as the functions
/// within are genericized to work for any encoder/decoder that is compatible
/// with that type.
pub trait EncDecExt: private::Sealed {
    /// Encode a value of this type with the provided [`Encoder`], returning
    /// either the cleartext or an error.
    ///
    /// # Errors
    ///
    /// Arbitrary errors may be returned by this function, depending on the
    /// encoder, via the [`CO2Error`](crate::error::CO2Error) type.
    fn encode_with<Ct, E: Encoder<Self, Ct>>(self, encoder: &mut E) -> CO2Result<Ct>
    where
        Self: Sized,
    {
        encoder.encode(self)
    }

    /// Decode a value of this type with the provided [`Decoder`], returning
    /// either the plaintext or an error.
    ///
    /// # Errors
    ///
    /// Arbitrary errors may be returned by this function, depending on the
    /// decoder, via the [`CO2Error`](crate::error::CO2Error) type.
    fn decode_with<Pt, D: Decoder<Pt, Self>>(self, decoder: &mut D) -> CO2Result<Pt>
    where
        Self: Sized,
    {
        decoder.decode(self)
    }
}

impl<F, Pt, Ct> Encoder<Pt, Ct> for F
where
    F: FnMut(Pt) -> CO2Result<Ct>,
{
    fn encode(&mut self, plaintext: Pt) -> CO2Result<Ct> {
        self(plaintext)
    }
}

impl<F, Pt, Ct> Decoder<Pt, Ct> for F
where
    F: FnMut(Ct) -> CO2Result<Pt>,
{
    fn decode(&mut self, ciphertext: Ct) -> CO2Result<Pt> {
        self(ciphertext)
    }
}

impl<T> EncDecExt for T {}

mod private {
    pub trait Sealed {}
    impl<T> Sealed for T {}
}

#[cfg(test)]
mod tests {
    use super::*;

    // A simple "identity" cipher for testing.
    struct Identity;

    impl<'a> Encoder<&'a str, &'a str> for Identity {
        fn encode(&mut self, plaintext: &'a str) -> CO2Result<&'a str> {
            Ok(plaintext)
        }
    }

    impl<'a> Decoder<&'a str, &'a str> for Identity {
        fn decode(&mut self, ciphertext: &'a str) -> CO2Result<&'a str> {
            Ok(ciphertext)
        }
    }

    #[test]
    fn decode_with_identity() {
        assert_eq!("abc".decode_with(&mut Identity).unwrap(), "abc");
    }

    #[test]
    fn encode_with_identity() {
        assert_eq!("abc".encode_with(&mut Identity).unwrap(), "abc")
    }
}
