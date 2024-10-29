pub mod impls;
#[cfg(feature = "varint")]
pub mod varint;

use std::string::FromUtf8Error;

pub use lbytes::{BytesReadExt, BytesWriteExt};
#[cfg(feature = "derive")]
pub use packet_binser_derive as derive;

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
	Lbytes(#[from] lbytes::Error),
}

pub trait Binser: Sized {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), Error>;
	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, Error>;
}
