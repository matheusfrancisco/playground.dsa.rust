use playground_dsa_rust::LinkedList;

use std::cell::RefCell;
use std::rc::Rc;

struct Node<T> {
    value: T,
}

fn main() {
    let list_option = Rc::new(RefCell::new(Node::<i32> { value: 4 }));
    let list_rc = Rc::clone(&list_option);
    let list_refcell = list_rc.borrow();
    let node = &*list_refcell;
    let node_value = node.value;
    println!("The value inside the Node is: {}", node_value);

    //let mut list = LinkedList::<i32>::new();
    //list.push_front(1);
    //list.push_front(2);
    //list.push_front(3);

    //println!("{}", list);
}
