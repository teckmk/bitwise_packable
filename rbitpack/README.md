
# rbitpack

The `rbitpack` crate provides a procedural macro for packing and unpacking boolean fields of a struct into various bit sizes. This can be particularly useful for reducing memory usage or performing bit-level operations.

## Features

- **Efficient Packing and Unpacking**: Pack multiple boolean fields into a single integer type (`u8`, `u16`, `u32`, `u64`) or a `Vec<u64>`.
- **Overflow Handling**: Control whether an overflow error should be triggered when the number of boolean fields exceeds the available bits.
- **Dynamic Bitfield Size**: Support for packing into a dynamic bitfield size with the `Bitfield` type from the `bitval` crate.

## Usage

To use the `rbitpack` macro, add it to your struct as follows:

```rust
use rbitpack::BitwisePackable;

#[derive(BitwisePackable)]
#[rbitpack(size = "i8", overflow = false)]
struct MyStruct {
    field1: bool,
    field2: bool,
    // Add more fields as needed
}
```

### Attributes

- `size`: Specifies the bit size for packing (`i8`, `i16`, `i32`, `i64`, or `auto` for dynamic sizing).
- `overflow`: Controls whether to panic on overflow (defaults to `false`).

### Auto Size

If you use `auto` for the `size` attribute, you need to install and import the `Bitfield` type from the `bitval` crate. Add `bitval` to your `Cargo.toml`:

```toml
[dependencies]
bitval = "0.1"
```

Then, import `Bitfield` in your Rust code:

```rust
use bitval::Bitfield;
```

## Examples

### Packing and Unpacking with `u8`

```rust
#[derive(BitwisePackable)]
#[rbitpack(size = "i8")]
struct Example {
    a: bool,
    b: bool,
    c: bool,
}

let example = Example { a: true, b: false, c: true };
let packed = Example::pack(&example);
let unpacked = Example::unpack(packed);
```

### Using Dynamic Bitfield Size

```rust
#[derive(BitwisePackable)]
#[rbitpack(size = "auto")]
struct DynamicExample {
    x: bool,
    y: bool,
    z: bool,
}

let example = DynamicExample { x: true, y: false, z: true };
let packed = DynamicExample::pack(&example);
let unpacked = DynamicExample::unpack(packed);
```

## License

This crate is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
