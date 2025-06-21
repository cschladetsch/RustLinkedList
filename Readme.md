# Rust Linked List 

A simple and efficient implementation of a linked list data structure in Rust.

## Overview

This project demonstrates how to implement a linked list in Rust, showcasing Rust's memory safety features and ownership model.

## Features

- Generic type support
- Basic linked list operations:
  - Push (add to front)
  - Pop (remove from front)
  - Peek (view front element)
  - Iterators (regular, mutable, and consuming)
  - Length calculation
  - Automatic memory cleanup

## How It Works

### Push Operation
```
Initial:  HEAD → None

Push(1):  HEAD → [1] → None

Push(2):  HEAD → [2] → [1] → None

Push(3):  HEAD → [3] → [2] → [1] → None
```

### Pop Operation
```
Initial:  HEAD → [3] → [2] → [1] → None
          
Pop():    Returns 3
          HEAD → [2] → [1] → None

Pop():    Returns 2
          HEAD → [1] → None

Pop():    Returns 1
          HEAD → None
```

### Visual Representation

```
┌─────────┐     ┌─────────┐     ┌─────────┐
│ HEAD    │ --> │ Node A  │ --> │ Node B  │ --> None
│ size: 3 │     │ elem: 3 │     │ elem: 2 │
└─────────┘     │ next: ──┼──>  │ next: ──┼──> ┌─────────┐
                └─────────┘     └─────────┘     │ Node C  │
                                                │ elem: 1 │
                                                │ next:None│
                                                └─────────┘
```

## Usage

```rust
use rust_linked_list::LinkedList;

fn main() {
    // Create a new linked list
    let mut list: LinkedList<i32> = LinkedList::new();

    // Add elements (pushes to front)
    list.push(1);  // [1]
    list.push(2);  // [2, 1]
    list.push(3);  // [3, 2, 1]

    // Check the front element
    assert_eq!(list.peek(), Some(&3));

    // Remove elements
    assert_eq!(list.pop(), Some(3));  // Returns 3, list is now [2, 1]
    
    // Iterate through the list
    for item in list.iter() {
        println!("Item: {}", item);
    }
    
    // Mutate elements
    for item in list.iter_mut() {
        *item *= 2;  // Double each element
    }
}
```

### More Examples

#### Working with Custom Types
```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

let mut people = LinkedList::new();
people.push(Person { name: "Alice".to_string(), age: 30 });
people.push(Person { name: "Bob".to_string(), age: 25 });
```

#### Using Iterators
```rust
let mut list = LinkedList::new();
list.push(1);
list.push(2);
list.push(3);

// Consuming iterator
let sum: i32 = list.into_iter().sum();  // list is consumed here
println!("Sum: {}", sum);  // Output: 6
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust_linked_list = { git = "https://github.com/cschladetsch/RustLinkedList.git" }
```

## Building

```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release
```

## Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_push_and_pop
```

### Test Coverage
```
- Basic operations (new, push, pop)
- Peek operations (peek, peek_mut)
- Iterators (iter, iter_mut, into_iter)
- Edge cases (empty lists, single elements)
- Large lists (10,000+ elements)
- Custom types and strings
- Zero-sized types
- Memory safety and drop behavior
```

## Performance Characteristics

| Operation | Time Complexity | Space Complexity |
|-----------|----------------|------------------|
| Push      | O(1)           | O(1)             |
| Pop       | O(1)           | O(1)             |
| Peek      | O(1)           | O(1)             |
| Length    | O(1)           | O(1)             |
| Drop      | O(n)           | O(1)             |

## Memory Layout

```
Stack                    Heap
┌─────────────┐         ┌──────────────┐
│ LinkedList  │         │ Node<T>      │
│ ┌─────────┐ │   ┌────▶│ ┌──────────┐ │
│ │ head    │─┼───┘     │ │ elem: T  │ │
│ │ size: 3 │ │         │ │ next: ───┼─┼──┐
│ └─────────┘ │         │ └──────────┘ │  │
└─────────────┘         └──────────────┘  │
                                          │
                        ┌──────────────┐  │
                        │ Node<T>      │◀─┘
                        │ ┌──────────┐ │
                        │ │ elem: T  │ │
                        │ │ next: ───┼─┼──┐
                        │ └──────────┘ │  │
                        └──────────────┘  │
                                          ▼
                                        None
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
- "Learning Rust With Entirely Too Many Linked Lists" guide
