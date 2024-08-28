use packet_binser_derive::Packet;

#[derive(Packet)]
#[header = 0x1]
struct HandshakePacket(u8);

#[derive(Packet)]
#[header = 0x2]
struct MeowPacket {
	message: String,
}
