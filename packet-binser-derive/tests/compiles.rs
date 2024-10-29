use packet_binser_derive::Binser;

#[derive(Binser)]
struct MeowData {
	message: String,
}

#[derive(Binser)]
enum TestPackets {
	A(i32),
	B(MeowData),
	C { a: i32, b: MeowData },
	D,
}
