use std::ops::{Deref, DerefMut};

use crate::{Binser, Error};
use zigzag::{ZigZagDecode, ZigZagEncode};

#[derive(Debug, Clone, Copy)]
pub struct Variable<T>(pub T);

impl<T> Deref for Variable<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T> DerefMut for Variable<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

macro_rules! impl_unsigned {
	($ty:ty) => {
		impl Binser for Variable<$ty> {
			fn serialize<B: lbytes::BytesWriteExt>(&self, buffer: &mut B) -> Result<(), Error> {
				let mut this = self.0;
				if this == 0 {
					buffer.write_u8(0)?;
					return Ok(());
				}

				while this >= 0b1000_0000 {
					let tmp = this & 0b0111_1111;
					let tmp = tmp | 0b1000_0000;
					buffer.write_u8(tmp as u8)?;

					this >>= 7;
				}

				buffer.write_u8((this & 0b0111_1111) as u8)?;
				Ok(())
			}

			fn deserialize<B: lbytes::BytesReadExt>(buffer: &mut B) -> Result<Self, Error> {
				let mut res = 0;
				let mut i = 0;

				loop {
					let tmp = buffer.read_u8()?;

					res |= ((tmp & 0b0111_1111) << 7 * i) as $ty;
					i += 1;

					if tmp & 0b1000_0000 == 0 {
						break;
					}
				}

				Ok(Variable(res))
			}
		}
	};
}

macro_rules! impl_signed {
	($bits:expr) => {
		::paste::paste! {
		impl Binser for Variable<[<i$bits>]> {
			fn serialize<B: lbytes::BytesWriteExt>(&self, buffer: &mut B) -> Result<(), Error> {
				Variable(self.0.zigzag_encode()).serialize(buffer)?;
				Ok(())
			}

			fn deserialize<B: lbytes::BytesReadExt>(buffer: &mut B) -> Result<Self, Error> {
				Ok(Variable(Variable::<[<u$bits>]>::deserialize(buffer)?.0.zigzag_decode()))
			}
		}
		}
	};
}

macro_rules! impl_for {
	($( $bits:expr ),*) => {
	$(
	::paste::paste! { impl_unsigned!([<u$bits>]); }
	::paste::paste! { impl_signed!($bits); }
	)*
	};
}

impl_for!(8, 16, 32, 64, 128);
