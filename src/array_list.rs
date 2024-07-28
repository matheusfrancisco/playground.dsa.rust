#![allow(dead_code)]
/*

 * Wrapper that uses arrays under the hood
 * push/pop/access has O(1)
 * enqueue/deque has O(N)
 * constructor specifies initial size

TypeScript example:
export default class ArrayList<T> {
    public length: number;
    constructor() {}
    prepend(item: T): void {}
    insertAt(item: T, idx: number): void {}
    append(item: T): void {}
    remove(item: T): T | undefined {}
    get(idx: number): T | undefined {}
    removeAt(idx: number): T | undefined {}
}

Array List
  - A dynamic array that can grow or shrink in size
  - It is a generic data structure that can store any type of data
  - It is implemented using a Vec
  - It has the following methods:
*/

use std::fmt::Debug;

type Array<T> = Vec<T>;

#[derive(Debug)]
struct ArrayList<T> {
    pub length: usize,
    inner: Array<T>,
    capacity: usize,
}

impl<T> ArrayList<T>
where
    T: Default + Clone + Debug,
{
    fn new(capacity: usize) -> ArrayList<T> {
        ArrayList {
            length: 0,
            capacity,
            inner: vec![T::default(); capacity],
        }
    }
    fn prepend(item: T) {}

    fn shrink(&mut self) {
        let prev = &self.inner;
        let new_capacity = self.capacity * 2; 
        self.capacity = new_capacity;
        let mut new = vec![T::default(); new_capacity];
        for i in 0..prev.len() {
            new[i] = prev[i].clone();
        }
        self.inner = new;
    }

    fn append(&mut self, item: T) {
        println!("Appending {item:?} to {self:?}");
        if self.inner.len() == self.length {
            self.shrink();
        }
        self.inner[self.length] = item;
        self.length += 1;
    }
}

mod tests {

    use super::*;

    #[test]
    fn test_array_list() {
        assert_eq!(1, 2);
    }

    #[test]
    fn test_array_list_append() {
        let mut arr = ArrayList::new(1);
        arr.append(1);
        assert_eq!(arr.length, 1);
    }

    #[test]
    fn test_array_list_append_shrink() {
        let mut arr = ArrayList::new(4);
        arr.append(1);
        arr.append(2);
        arr.append(3);
        arr.append(4);
        arr.append(4);
        assert_eq!(arr.length, 5);
        assert_eq!(arr.capacity, 8);
    }
}
