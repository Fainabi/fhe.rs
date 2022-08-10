//! Traits used for the BFV homomorphic encryption scheme.

use crate::ciphertext::Ciphertext;
use crate::parameters::BfvParameters;
use crate::plaintext::{Encoding, Plaintext};
use std::rc::Rc;

/// Encode values into [`Plaintext`].
pub trait Encoder<T>
where
	Self: Sized,
{
	/// The type of errors.
	type Error;

	/// Attempt to encode the `value` with the specified [`Encoding`] and [`BfvParameters`].
	fn try_encode(
		value: T,
		encoding: Encoding,
		par: &Rc<BfvParameters>,
	) -> Result<Self, Self::Error>;
}

/// Decode [`Plaintext`] values.
pub trait Decoder
where
	Self: Sized,
{
	/// The type of errors.
	type Error;

	/// Attempt to decode the [`Plaintext`], with an optional [`Encoding`].
	fn try_decode<E>(a: &Plaintext, encoding: E) -> Result<Self, Self::Error>
	where
		E: Into<Option<Encoding>>;
}

/// Encrypt a [`Plaintext`] into a [`Ciphertext`].
pub trait Encryptor {
	/// The type of errors.
	type Error;

	/// Encrypt a [`Plaintext`].
	fn encrypt(&self, plaintext: &Plaintext) -> Result<Ciphertext, Self::Error>;
}

/// Decrypt a [`Ciphertext`] into a [`Plaintext`].
pub trait Decryptor {
	/// The type of errors.
	type Error;

	/// Decrypt a [`Ciphertext`].
	fn decrypt(&self, ciphertext: &Ciphertext) -> Result<Plaintext, Self::Error>;
}