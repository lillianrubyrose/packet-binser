use packet_binser_derive::Binser;

#[derive(Binser)]
struct MeowData {
	message: String,
}

#[derive(Binser)]
#[repr(u8)]
enum TestPackets {
	A(i32),
	B(MeowData) = 4,
	C { a: i32, b: MeowData },
	D,
}
