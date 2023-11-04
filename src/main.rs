use data_structure_in_rust::LinkedList;

fn main() {
    let mut list = LinkedList::<i32>::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    println!("{}", list);
}
