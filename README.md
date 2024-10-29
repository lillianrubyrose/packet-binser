# packet-binser

`packet-binser` is a Rust library for simple (de)serialization of network packets.

It provides default implementations for predictibly sized primitive types as well as common std types.

## Features
- Derive macros available to automatically implement serialization and deserialization. (Enabled with the `derive` feature.)
- Supports variable-length integer encoding with `packet_binser::varint::Variable<T>`. Signed integers are supported via zigzag encoding. (Enabled with the `varint` feature.)

## Getting Started

### Add to `Cargo.toml`

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
# or latest version
packet-binser = "0.2" # or packet-binser = { version = "0.2", features = [...] }
```

### Example Usage (Without derive)

```rust
use packet_binser::{Binser, BytesReadExt, BytesWriteExt, Error};

struct HandshakePacket(u8);

impl Binser for HandshakePacket {
    fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), Error> {
        buffer.write_u8(0x1)?; // packet id, not read in deserialize since it should be read elsewhere
        self.0.serialize(buffer)?;
        Ok(())
    }

    fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, Error> {
        Ok(Self(u8::deserialize(buffer)?))
    }
}
```

### Example Usage (With derive)

```rust
use packet_binser::derive::Binser;

#[derive(Binser)]
#[repr(u8)]
enum ClientPackets {
    Handshake {
        id: u8,
    } = 0x1,
}
```

## Default Implementations

- `u8`, `u16`, `u32`, `u64`, `u128`
- `i8`, `i16`, `i32`, `i64`, `i128`
- `f32`, `f64`
- `bool`
- `std::string::String`
- `std::option::Option<T: Binser>`
- `std::vec::Vec<T: Binser>`
- `[T; N] where T: Binser`
- `packet_binser::varint::Variable`

## License

This project is dual licensed under both the [MIT License](./LICENSE-MIT) and [Apache License 2.0](./LICENSE-APACHE).

---

Feel free to [open an issue](https://github.com/lillianrubyrose/packet-binser/issues/new) if you encounter any problems or have suggestions.
