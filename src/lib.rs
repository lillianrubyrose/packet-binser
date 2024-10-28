pub use lbytes;
pub use lbytes::{BytesReadExt, BytesWriteExt};

pub mod impls;

#[cfg(feature = "varint")]
pub mod varint;

#[cfg(feature = "derive")]
pub use packet_binser_derive as derive;

pub trait PacketSerde: Sized {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), lbytes::Error>;
	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, lbytes::Error>;
}
