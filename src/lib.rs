#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![doc = include_str!("../README.md")]

pub mod impls;

use std::string::FromUtf8Error;

pub use lbytes::{BytesReadExt, BytesWriteExt};
#[cfg(feature = "derive")]
pub use packet_binser_derive as proc;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Invalid packet data: {0}")]
	InvalidPacketData(String),
	#[error(transparent)]
	FromUtf8Error(#[from] FromUtf8Error),
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	TryFromInt(#[from] std::num::TryFromIntError),
	#[error(transparent)]
	Lbytes(#[from] lbytes::Error),
}

pub trait Binser: Sized {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), Error>;
	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, Error>;
}

#[cfg(test)]
mod tests {
	use super::{Binser, Error};
	use std::{fmt::Debug, io::Cursor};

	fn binserde<T: Binser + PartialEq + Debug>(value: T) -> Result<(), Error> {
		let mut buffer = Vec::new();
		value.serialize(&mut buffer)?;
		let de = T::deserialize(&mut Cursor::new(buffer))?;
		Ok(assert_eq!(value, de, "binserde failed for {:?}", value))
	}

	#[test]
	fn test_binserde() -> Result<(), Error> {
		binserde(true)?;
		binserde(false)?;
		binserde(69_u32)?;
		binserde(-69_i32)?;
		binserde(4.2_f32)?;

		binserde(Some(42_u8))?;
		binserde(None::<u8>)?;
		binserde(Some(Some(84_u16)))?;
		binserde(Some(None::<u16>))?;

		binserde(String::from("Hello, world!"))?;
		binserde(String::from("❤️🎃😊"))?;
		binserde(String::from(""))?;

		binserde([-69_i8, 0, 1, 2])?;
		binserde(vec![-69_i32, 0, 1, 2])?;
		binserde(Vec::<i32>::new())?;
		binserde(vec![u8::MAX, u8::MIN, 127, 128])?;

		binserde(u64::MAX)?;
		binserde(i64::MIN)?;

		Ok(())
	}
}
