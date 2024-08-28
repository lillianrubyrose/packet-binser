pub use lbytes;
pub use lbytes::{BytesReadExt, BytesWriteExt};

pub mod impls;
pub mod varint;

pub trait PacketSerde: Sized {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), lbytes::Error>;
	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, lbytes::Error>;
}
