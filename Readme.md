# Rust Linked List 

A simple and efficient implementation of a linked list data structure in Rust.

## Overview

This project demonstrates how to implement a linked list in Rust, showcasing Rust's memory safety features and ownership model.

## Features

- Generic type support
- Basic linked list operations:
  - Push front/back
  - Pop front/back
  - Insert
  - Remove
  - Length calculation
  - Iterator implementation

## Usage

```rust
use rust_linked_list::LinkedList;

fn main() {
    // Create a new linked list
    let mut list: LinkedList<i32> = LinkedList::new();

    // Add elements
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    // Iterate through the list
    for item in list.iter() {
        println!("{}", item);
    }
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust_linked_list = { git = "https://github.com/cschladetsch/RustLinkedList.git" }
```

## Building

```bash
cargo build
cargo test
```

## Testing

Run the test suite:

```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Rust Programming Language Documentation
- Various linked list implementations in Rust community
