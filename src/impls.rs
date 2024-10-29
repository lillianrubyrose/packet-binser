#[cfg(feature = "variable-width-lengths")]
use crate::varint::Variable;
use crate::{Binser, Error};

use lbytes::{BytesReadExt, BytesWriteExt};

macro_rules! _impl_num {
	($ty:ty) => {
		::paste::paste! {
			impl Binser for $ty {
				fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), Error> {
					buffer.[<write_$ty>](*self)?;
					Ok(())
				}

				fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, Error> {
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

impl Binser for bool {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), Error> {
		buffer.write_u8(u8::from(*self))?;
		Ok(())
	}

	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, Error> {
		Ok(match buffer.read_u8()? {
			0 => false,
			1.. => true,
		})
	}
}

impl<T: Binser, const N: usize> Binser for [T; N] {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), Error> {
		#[cfg(feature = "variable-with-lengths")]
		Variable(N as u64).serialize(buffer)?;
		#[cfg(not(feature = "variable-with-lengths"))]
		(N as u32).serialize(buffer)?;

		for ele in self {
			ele.serialize(buffer)?;
		}

		Ok(())
	}

	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, Error> {
		#[cfg(feature = "variable-width-lengths")]
		let len = *Variable::<u64>::deserialize(buffer)?;
		#[cfg(not(feature = "variable-width-lengths"))]
		let len = u32::deserialize(buffer)?;
		let mut vec: Vec<T> = Vec::with_capacity(len as usize);

		for _ in 0..len {
			vec.push(T::deserialize(buffer)?);
		}

		Ok(vec
			.try_into()
			.unwrap_or_else(|_| unreachable!("exepct to be optimized out by the compiler")))
	}
}

impl<T: Binser> Binser for Vec<T> {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), Error> {
		#[cfg(feature = "variable-with-lengths")]
		Variable(self.len() as u64).serialize(buffer)?;
		#[cfg(not(feature = "variable-with-lengths"))]
		(self.len() as u32).serialize(buffer)?;

		for ele in self {
			ele.serialize(buffer)?;
		}

		Ok(())
	}

	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, Error> {
		#[cfg(feature = "variable-width-lengths")]
		let len = *Variable::<u64>::deserialize(buffer)?;
		#[cfg(not(feature = "variable-width-lengths"))]
		let len = u32::deserialize(buffer)?;
		let mut vec: Vec<T> = Vec::with_capacity(len as usize);

		for _ in 0..len {
			vec.push(T::deserialize(buffer)?);
		}

		Ok(vec)
	}
}

impl<T: Binser> Binser for Option<T> {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), Error> {
		self.is_some().serialize(buffer)?;
		if let Some(t) = self {
			t.serialize(buffer)?;
		}
		Ok(())
	}

	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, Error> {
		Ok(if bool::deserialize(buffer)? {
			Some(T::deserialize(buffer)?)
		} else {
			None
		})
	}
}

impl Binser for String {
	fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), Error> {
		#[cfg(feature = "variable-with-lengths")]
		Variable(self.len() as u64).serialize(buffer)?;
		#[cfg(not(feature = "variable-with-lengths"))]
		(self.len() as u32).serialize(buffer)?;
		buffer.write_all(self.as_bytes())?;
		Ok(())
	}

	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, Error> {
		Ok(String::from_utf8(Vec::<u8>::deserialize(buffer)?)?)
	}
}
