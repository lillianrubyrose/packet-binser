use packet_binser_derive::{Packet, PacketSerde};

#[derive(Packet)]
#[header = 0x1]
struct HandshakePacket(u8);

#[derive(PacketSerde)]
struct MeowData {
	message: String,
}

#[derive(Packet)]
#[header = 0x2]
struct MeowPacket {
	data: MeowData,
}
