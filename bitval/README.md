# Bitfield

A `Bitfield` is a data structure that allows efficient storage and manipulation of individual bits. This implementation uses a `Vec<u64>` to store the bits, allowing for compact storage and fast access.

## Overview

The `Bitfield` struct provides methods to create a bitfield of a specified size, set the value of individual bits, and get the value of individual bits.

## Usage

### Creating a Bitfield

To create a new `Bitfield` with a specific number of bits, use the `new` method:

```rust
let size = 128;
let mut bitfield = Bitfield::new(size);
```

This will create a `Bitfield` with 128 bits, all initially set to 0.

### Setting a Bit

To set the value of a specific bit, use the `set` method:

```rust
bitfield.set(5, true); // Set the 5th bit to 1
bitfield.set(10, false); // Set the 10th bit to 0
```

If the provided index is out of bounds, the method will panic with an "Index out of bounds" message.

### Getting a Bit

To get the value of a specific bit, use the `get` method:

```rust
let value = bitfield.get(5); // Get the value of the 5th bit
println!("Value of 5th bit: {}", value);
```

If the provided index is out of bounds, the method will panic with an "Index out of bounds" message.

## Example

Here's an example demonstrating how to use the `Bitfield` struct:

```rust
use bitval::Bitfield;

fn main() {
    let size = 128;
    let mut bitfield = Bitfield::new(size);

    bitfield.set(5, true);
    bitfield.set(10, false);

    let value = bitfield.get(5);
    println!("Value of 5th bit: {}", value); // Output: Value of 5th bit: true
}
```

### Cargo.toml

To include the `bitval` crate in your project, add the following entry to your `Cargo.toml` file:

```toml
[dependencies]
bitval = "0.1.0" // Replace with the current version
```

This will compile and run the tests, verifying that the `Bitfield` implementation behaves correctly.

## License

This project is licensed under the MIT License.
