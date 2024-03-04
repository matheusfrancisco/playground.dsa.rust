use core::fmt;
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{Display, Formatter},
    rc::Rc,
};

/*
* head                        tail
    |                           |
    v                           v
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
#[derive(Debug, Clone)]
pub struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { data, next: None }))
    }
}

pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    length: usize,
}

pub struct IntoIter<T>(LinkedList<T>);

pub struct IterMut<T> {
    next: Option<NodeRef<T>>,
}

type NodeRef<T> = Rc<RefCell<Node<T>>>;
pub struct Iter<T> {
    next: Option<NodeRef<T>>,
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.head {
            Some(node) => write!(f, "{}", node.borrow()),
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
            Some(node) => write!(f, "{}, -> {}", self.data, node.borrow()),
            None => write!(f, "{}", self.data),
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_head = Node::new(data);
        match self.head.take() {
            Some(head) => {
                new_head.borrow_mut().next = Some(head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone()); // +1 new_head
                self.head = Some(new_head);
            }
        }
        self.length += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let new = Node::new(data);
        match self.tail.take() {
            Some(tail) => {
                new.borrow_mut().next = Some(tail);
                self.tail = Some(new);
            }
            None => {
                self.tail = Some(new.clone()); // +1 new_head
                self.head = Some(new);
            }
        }
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    pub fn get(&self, index: usize) -> Ref<T> {
        let current = self.head.as_ref();
        let mut i = 0;

        if index >= self.length {
            panic!("index out of bound");
        }
        if index < i {
            panic!("index negative not allowed");
        }

        while let Some(node) = current {
            if i == index {
                return Ref::map(node.borrow(), |node| &node.data);
            }
            i += 1;
        }
        panic!("index negative not allowed");
    }

    pub fn peek(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.data))
    }

    pub fn peek_mut(&mut self) -> Option<RefMut<'_, T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |n| &mut n.data))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().cloned(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_ref().cloned(),
        }
    }
}

impl<'a, T> Iterator for IterMut<T> {
    type Item = NodeRef<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.next.as_ref().cloned() {
            self.next = cur.borrow_mut().next.clone();
            return Some(cur);
        }
        None
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<T> {
    type Item = NodeRef<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.next.as_ref().cloned() {
            self.next = cur.borrow().next.clone();
            return Some(cur);
        }
        None
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(node) = current {
            current = node.borrow_mut().next.take();
        }
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
    fn push_front() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(1);
        assert_eq!(list.length, 1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        if let Some(head) = &list.head {
            let h = head.borrow();
            let node = &*h;
            assert_eq!(node.data, 4);
        }

        if let Some(head) = &list.tail {
            let h = head.borrow();
            let node = &*h;
            assert_eq!(node.data, 1);
        } else {
            panic!("tail is none");
        }

        assert_eq!(list.length, 4);
    }

    #[test]
    fn push_back() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        assert_eq!(list.length, 1);
        list.push_back(4);

        if let Some(head) = &list.tail {
            let h = head.borrow();
            let node = &*h;
            assert_eq!(node.data, 4);
        } else {
            panic!("tail is none");
        }

        if let Some(head) = &list.head {
            let h = head.borrow();
            let node = &*h;
            assert_eq!(node.data, 1);
        }
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
    fn peek_and_peek_mut() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(11);
        let p = list.peek();
        assert_eq!(&*p.unwrap(), &11);
        list.push_front(10);
        assert_eq!(&*list.peek().unwrap(), &10);
        assert_eq!(&*list.peek_mut().unwrap(), &mut 10);
        list.peek_mut().map(|mut v| *v = 100);
        assert_eq!(&*list.peek_mut().unwrap(), &mut 100);
        assert_eq!(list.pop(), Some(100));
        assert_eq!(&*list.peek().unwrap(), &11);
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(13);
        list.push_front(14);
        list.push_front(15);
        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap().borrow().data, 15);
        assert_eq!(iter.next().unwrap().borrow().data, 14);
        assert_eq!(iter.next().unwrap().borrow().data, 13);
        //TODO: improve iter to return None
        //assert_eq!(iter.next().unwrap().borrow().data, None);
    }

    #[test]
    fn iter_mut() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(13);
        list.push_front(14);
        list.push_front(15);
        let mut iter = list.iter_mut();
        // TODO: fix iter_mut
        assert_eq!(iter.next().unwrap().borrow_mut().data, 15);
    }
}
