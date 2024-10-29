use packet_binser_derive::{Binser, Packet};

#[derive(Packet)]
#[header = 0x1]
struct HandshakePacket(u8);

#[derive(Binser)]
struct MeowData {
	message: String,
}

#[derive(Packet)]
#[header = 0x2]
struct MeowPacket {
	data: MeowData,
}

#[derive(Binser)]
enum TestPackets {
	A(i32),
	B(MeowData),
	C { a: i32, b: MeowData },
	D,
}
