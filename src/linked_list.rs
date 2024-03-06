type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn push() {
        let mut list = List::<i32>::new();
        list.push(1);
        list.push(3);
        list.push(4);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(1));
    }

    //    #[test]
    //    fn push_front() {
    //        let mut list = LinkedList::<i32>::new();
    //        list.push_front(1);
    //        assert_eq!(list.length, 1);
    //        list.push_front(2);
    //        list.push_front(3);
    //        list.push_front(4);
    //        if let Some(head) = &list.head {
    //            let h = head.borrow();
    //            let node = &*h;
    //            assert_eq!(node.data, 4);
    //        }
    //
    //        if let Some(head) = &list.tail {
    //            let h = head.borrow();
    //            let node = &*h;
    //            assert_eq!(node.data, 1);
    //        } else {
    //            panic!("tail is none");
    //        }
    //
    //        assert_eq!(list.length, 4);
    //    }
    //
    //    #[test]
    //    fn push_back() {
    //        let mut list = LinkedList::<i32>::new();
    //        list.push_back(1);
    //        assert_eq!(list.length, 1);
    //        list.push_back(4);
    //
    //        if let Some(head) = &list.tail {
    //            let h = head.borrow();
    //            let node = &*h;
    //            assert_eq!(node.data, 4);
    //        } else {
    //            panic!("tail is none");
    //        }
    //
    //        if let Some(head) = &list.head {
    //            let h = head.borrow();
    //            let node = &*h;
    //            assert_eq!(node.data, 1);
    //        }
    //    }
}
