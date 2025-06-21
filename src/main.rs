use rust_linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    
    println!("Creating a new linked list and pushing elements...");
    list.push(1);
    list.push(2);
    list.push(3);
    
    println!("List length: {}", list.len());
    println!("Is empty: {}", list.is_empty());
    
    println!("\nPeeking at the head: {:?}", list.peek());
    
    println!("\nIterating through the list:");
    for value in list.iter() {
        println!("  {}", value);
    }
    
    println!("\nPopping elements:");
    while let Some(value) = list.pop() {
        println!("  Popped: {}", value);
    }
    
    println!("\nList is now empty: {}", list.is_empty());
}
