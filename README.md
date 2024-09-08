# packet-binser

`packet-binser` is a Rust library with a main goal of (de)serialization of network packets. It can be used for any binary data as a `#[header]` is not required.

It provides default implementations for common types and allows for easy implementations with an optional derive macro.

## Features
- Default implementations for primitive types, `Option`, `Vec`, arrays, and `String`.
- Derive macros available to automatically implement serialization and deserialization.
- Supports variable-length integer encoding with `packet_binser::varint::Variable<T>`. Signed integers are supported via zigzag encoding.

## Getting Started

### Add to `Cargo.toml`

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
packet-binser = "0.1.0" # or packet-binser = { version = "0.1.0", features = ["derive"] }
```

### Example Usage (Without derive)

```rust
use packet_binser::{PacketSerde, BytesReadExt, BytesWriteExt, lbytes};

struct HandshakePacket(u8);

impl PacketSerde for HandshakePacket {
   fn serialize<B: BytesWriteExt>(&self, buffer: &mut B) -> Result<(), lbytes::Error> {
      buffer.write_u8(0x1)?; // packet id, not read in deserialize since it should be read elsewhere
      self.0.serialize(buffer)?;
      Ok(())
   }
	fn deserialize<B: BytesReadExt>(buffer: &mut B) -> Result<Self, lbytes::Error> {
		Ok(Self(u8::deserialize(buffer)?)
	}
}
```

### Example Usage (With derive)

```rust
use packet_binser::derive::PacketSerde;

#[derive(PacketSerde)]
#[header(0x1)]
struct HandshakePacket(u8);
```

## Implementations

We provide implementations of the `PacketSerde` trait for primitive types, arrays, vectors, `Option`, and `String`.

### Builtin Types

The following builtin types are supported:

- `u8`, `u16`, `u32`, `u64`, `u128`
- `i8`, `i16`, `i32`, `i64`, `i128`
- `f32`, `f64`
- `bool`
- `std::string::String`

### Arrays and Vectors

Requires the inner type to impl `PacketSerde`.

### Options

Requires the inner type to impl `PacketSerde`.

## License

This project is dual licensed under both the [MIT License](./LICENSE-MIT) and [Apache License 2.0](./LICENSE-APACHE).

---

Feel free to [open an issue](https://github.com/lillianrubyrose/packet-binser/issues/new) at if you encounter any problems or have suggestions.
