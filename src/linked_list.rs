use core::fmt;
use std::fmt::{Display, Formatter};

/*
* head
    |
    v
+--------+   +--------+   +--------+
|        |   |        |   |        |
| node 0 |-->| node 1 |-->| node 2 |--> NULL
|        |   |        |   |        |
+--------+   +--------+   +--------+
*  Implementations
*  1. push_front
*   time complexity: O(1)
*  2. pop
*    time complexity: O(1)
*  3. get
*    time complexity: O(n)
*  4. peek
*    time complexity: O(1)
*  5. peek_mut
*    time complexity: O(1)
* */

#[derive(Clone, Debug)]
struct Node<T> {
    pub data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

pub struct IntoIter<T>(LinkedList<T>);
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.head {
            Some(node) => write!(f, "{}", node.as_ref()),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.next {
            Some(node) => write!(f, "{}, -> {}", self.data, node.as_ref()),
            None => write!(f, "{}", self.data),
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.data
        })
    }

    pub fn get(&self, index: usize) -> &T {
        let mut current = &self.head;
        let mut i = 0;

        if index >= self.length {
            panic!("Index out of bounds");
        }
        while let Some(node) = current {
            if i == index {
                return &node.data;
            }
            current = &node.next;
            i += 1;
        }
        panic!("Index out of bounds");
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|n| &mut n.data)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
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

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|n| {
            self.next = n.next.as_deref();
            &n.data
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn check_length() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(1);
        assert_eq!(list.length, 1);
        list.push_front(1);
        list.push_front(1);
        list.push_front(1);
        assert_eq!(list.length, 4);
    }

    #[test]
    fn get_by_index() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(10);
        let item = list.get(0);
        assert_eq!(*item, 10);
    }

    #[test]
    fn pop_head() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(10);
        let item = list.pop();
        assert_eq!(item, Some(10));
        let item = list.pop();
        assert_eq!(item, None);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(10);
        assert_eq!(*list.get(1), 10);
    }

    #[test]
    fn get_ref_head() {
        let mut list = LinkedList::<i32>::new();
        let ref_h = list.peek();
        assert_eq!(ref_h, None);

        list.push_front(11);
        list.push_front(10);
        assert_eq!(list.peek(), Some(&10));
        assert_eq!(list.peek_mut(), Some(&mut 10));
        list.peek_mut().map(|v| *v = 100);
        assert_eq!(list.peek(), Some(&100));
        assert_eq!(list.pop(), Some(100));
        assert_eq!(list.peek(), Some(&11));
    }
}
