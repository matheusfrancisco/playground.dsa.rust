use core::fmt;
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{Display, Formatter},
    rc::Rc,
};

pub struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    length: usize,
}
