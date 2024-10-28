#[cfg(feature = "varint")]
use crate::varint::Variable;
use crate::PacketSerde;

use lbytes::{BytesReadExt, BytesWriteExt};

macro_rules! _impl_num {
	($ty:ty) => {
		::paste::paste! {
			impl PacketSerde for $ty {
				fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), lbytes::Error> {
					buffer.[<write_$ty>](*self)?;
					Ok(())
				}

				fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, lbytes::Error> {
					Ok(buffer.[<read_$ty>]()?)
				}
			}
		}
	};
}

macro_rules! impl_nums {
    ($( $ty:ty ),*) => {
        $( _impl_num!($ty); )*
    };
}

impl_nums!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl PacketSerde for bool {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), lbytes::Error> {
		buffer.write_u8(*self as u8)?;
		Ok(())
	}

	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, lbytes::Error> {
		Ok(match buffer.read_u8()? {
			0 => false,
			1 => true,
			_ => todo!("Error or default to false?"),
		})
	}
}

impl<T: PacketSerde, const N: usize> PacketSerde for [T; N] {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), lbytes::Error> {
		#[cfg(feature = "variable-with-lengths")]
		Variable(N as u64).serialize(buffer)?;
		#[cfg(not(feature = "variable-with-lengths"))]
		(N as u64).serialize(buffer)?;

		for ele in self {
			ele.serialize(buffer)?;
		}

		Ok(())
	}

	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, lbytes::Error> {
		#[cfg(feature = "variable-width-lengths")]
		let len = *Variable::<u64>::deserialize(buffer)?;
		#[cfg(not(feature = "variable-width-lengths"))]
		let len = u64::deserialize(buffer)?;
		let mut vec: Vec<T> = Vec::with_capacity(len as usize);

		for _ in 0..len {
			vec.push(T::deserialize(buffer)?);
		}

		let Ok(res) = vec.try_into() else {
			// Instead of unwrap to avoid requiring Debug
			unreachable!("This should be optimized by the compiler")
		};

		Ok(res)
	}
}

impl<T: PacketSerde> PacketSerde for Vec<T> {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), lbytes::Error> {
		#[cfg(feature = "variable-with-lengths")]
		Variable(self.len() as u64).serialize(buffer)?;
		#[cfg(not(feature = "variable-with-lengths"))]
		(self.len() as u64).serialize(buffer)?;

		for ele in self {
			ele.serialize(buffer)?;
		}

		Ok(())
	}

	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, lbytes::Error> {
		#[cfg(feature = "variable-width-lengths")]
		let len = *Variable::<u64>::deserialize(buffer)?;
		#[cfg(not(feature = "variable-width-lengths"))]
		let len = u64::deserialize(buffer)?;
		let mut vec: Vec<T> = Vec::with_capacity(len as usize);

		for _ in 0..len {
			vec.push(T::deserialize(buffer)?);
		}

		Ok(vec)
	}
}

impl<T: PacketSerde> PacketSerde for Option<T> {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), lbytes::Error> {
		self.is_some().serialize(buffer)?;
		if let Some(t) = self {
			t.serialize(buffer)?;
		}
		Ok(())
	}

	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, lbytes::Error> {
		Ok(if bool::deserialize(buffer)? {
			Some(T::deserialize(buffer)?)
		} else {
			None
		})
	}
}

impl PacketSerde for String {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), lbytes::Error> {
		#[cfg(feature = "variable-with-lengths")]
		Variable(self.len() as u64).serialize(buffer)?;
		#[cfg(not(feature = "variable-with-lengths"))]
		(self.len() as u64).serialize(buffer)?;
		buffer.write_all(self.as_bytes())?;
		Ok(())
	}

	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, lbytes::Error> {
		Ok(String::from_utf8(Vec::<u8>::deserialize(buffer)?)?)
	}
}
