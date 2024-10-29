use std::io::Cursor;

use packet_binser::Binser;
use packet_binser_derive::Binser;

#[derive(Binser)]
struct Data {
	a: u8,
	msg: String,
	b: u8,
}

#[derive(Binser, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ClientType {
	Good,
	Bad,
}

#[derive(Binser, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum ClientPackets {
	Handshake {
		id: u32,
		group: String,
		client_type: ClientType,
	} = 69,
}

#[test]
fn test_struct() -> Result<(), Box<dyn std::error::Error>> {
	let mut buffer = Cursor::new(Vec::new());
	let data = Data {
		a: 1,
		msg: "Hello".to_string(),
		b: 2,
	};
	data.serialize(&mut buffer)?;
	buffer.set_position(0);

	#[cfg(not(feature = "variable-width-lengths"))]
	assert_eq!(
		// 1u8
		// 5u32 BE
		// "Hello" ascii
		// 2u8
		&[1, 0, 0, 0, 5, 72, 101, 108, 108, 111, 2],
		buffer.into_inner().as_slice()
	);
	#[cfg(feature = "variable-width-lengths")]
	assert_eq!(
		// 1u8
		// 5 u32 VarInt BE
		// "Hello" ascii
		// 2u8
		&[1, 5, 72, 101, 108, 108, 111, 2],
		buffer.into_inner().as_slice()
	);

	Ok(())
}

#[test]
fn test_packet() -> Result<(), Box<dyn std::error::Error>> {
	let mut buffer = Cursor::new(Vec::new());
	let data = ClientPackets::Handshake {
		id: 1,
		group: "Hello".to_string(),
		client_type: ClientType::Good,
	};
	data.serialize(&mut buffer)?;
	buffer.set_position(0);

	#[cfg(not(feature = "variable-width-lengths"))]
	assert_eq!(
		// 69u16 BE packet id
		// 1u32 BE
		// 5u32 BE string length
		// "Hello" ascii
		// 0u16 BE client type enum variant
		&[0, 69, 0, 0, 0, 1, 0, 0, 0, 5, 72, 101, 108, 108, 111, 0, 0],
		buffer.clone().into_inner().as_slice()
	);
	#[cfg(feature = "variable-width-lengths")]
	assert_eq!(
		// 69u16 BE packet id
		// 1u32 BE
		// 5u32 BE VarInt string length
		// "Hello" ascii
		// 0u16 BE client type enum variant
		&[0, 69, 0, 0, 0, 1, 5, 72, 101, 108, 108, 111, 0, 0],
		buffer.clone().into_inner().as_slice()
	);

	let data_deserialized = ClientPackets::deserialize(&mut buffer)?;
	assert_eq!(data, data_deserialized);

	Ok(())
}
