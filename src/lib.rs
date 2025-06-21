use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> LinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_and_pop() {
        let mut list = LinkedList::new();
        
        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());
        
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        
        list.push(1);
        list.push(2);
        
        assert_eq!(list.peek(), Some(&2));
        list.pop();
        assert_eq!(list.peek(), Some(&1));
    }

    #[test]
    fn test_peek_mut() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        
        if let Some(value) = list.peek_mut() {
            *value = 42;
        }
        
        assert_eq!(list.peek(), Some(&42));
    }

    #[test]
    fn test_into_iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        for value in list.iter_mut() {
            *value *= 2;
        }
        
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&2));
    }

    #[test]
    fn test_default() {
        let list: LinkedList<i32> = LinkedList::default();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_drop() {
        let mut list = LinkedList::new();
        for i in 0..1000 {
            list.push(i);
        }
    }

    #[test]
    fn test_mixed_operations() {
        let mut list = LinkedList::new();
        
        assert!(list.is_empty());
        
        list.push(1);
        list.push(2);
        assert_eq!(list.len(), 2);
        
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop(), Some(2));
        
        list.push(3);
        list.push(4);
        assert_eq!(list.len(), 3);
        
        let collected: Vec<_> = list.iter().copied().collect();
        assert_eq!(collected, vec![4, 3, 1]);
        
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn test_with_strings() {
        let mut list = LinkedList::new();
        
        list.push(String::from("hello"));
        list.push(String::from("world"));
        
        assert_eq!(list.peek(), Some(&String::from("world")));
        assert_eq!(list.pop(), Some(String::from("world")));
        assert_eq!(list.pop(), Some(String::from("hello")));
        assert_eq!(list.pop(), None);
    }
}