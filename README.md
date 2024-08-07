# Bitwise Packable Workspace

This workspace contains two crates: `bitpack` and `bitval`. Together, they provide efficient bit manipulation and packing functionalities for Rust projects.

## Crates

### [bitpack](bitpack/README.md)

The `bitpack` crate provides macros and utilities for packing and unpacking boolean fields into various bit sizes. It is designed to work with structs, allowing for efficient bitwise operations.

### [bitval](bitval/README.md)

The `bitval` crate provides the `Bitfield` struct, which allows for efficient storage and manipulation of individual bits. It is used as the underlying data structure for the `bitpack` crate.

## Getting Started

### Cloning the Repository

To get started, clone the repository:

```sh
git clone https://github.com/teckmk/bitwise_packable.git
cd bitwise_packable
```

### Building the Workspace

To build the entire workspace, run:

```sh
cargo build
```

### Running Tests

Each crate contains its own set of tests. To run all tests in the workspace, use:

```sh
cargo test
```

## bitpack Crate

### Overview

The `bitpack` crate provides macros for packing and unpacking boolean fields into various bit sizes. It supports custom attributes to handle overflow values and dynamic bit sizes.

### Usage

Add `bitpack` to your `Cargo.toml`:

```toml
[dependencies]
bitpack = "0.1.0" // Replace with the current version
```

### Example

```rust
use bitpack::BitwisePackable;

#[derive(BitwisePackable)]
struct Example {
    field1: bool,
    field2: bool,
    field3: bool,
    // ...
}
```

### Running Tests

To run tests for `bitpack`:

```sh
cd bitpack
cargo test
```

For more details, see the [bitpack README](bitpack/README.md).

## bitval Crate

### Overview

The `bitval` crate provides the `Bitfield` struct for efficient bit storage and manipulation. It supports setting and getting individual bits and is used by the `bitpack` crate.

### Usage

Add `bitval` to your `Cargo.toml`:

```toml
[dependencies]
bitval = "0.1.0" // Replace with the current version
```

### Example

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

### Running Tests

To run tests for `bitval`:

```sh
cd bitval
cargo test
```

For more details, see the [bitval README](bitval/README.md).

## Development

### Adding New Features

1. Create a new branch for your feature:
   ```sh
   git checkout -b feature-name
   ```

2. Make your changes and commit them:
   ```sh
   git commit -am "Add new feature"
   ```

3. Push your branch to the remote repository:
   ```sh
   git push origin feature-name
   ```

4. Create a pull request on GitHub.

### Contributing

Contributions are welcome! Please open an issue or submit a pull request with your changes. Make sure to include tests for any new features or bug fixes.

### License

This project is licensed under the MIT License.

## Contact

For questions or feedback, please contact (Abdullah Yasir)[https://linkedin.com/in/abdullah-yasir-itech].
