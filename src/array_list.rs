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
pub struct ArrayList<T> {
    pub length: usize,
    inner: Array<T>,
    capacity: usize,
}

impl<T> ArrayList<T>
where
    T: Default + Clone + Debug,
{
    pub fn new(capacity: usize) -> ArrayList<T> {
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

    pub fn append(&mut self, item: T) {
        println!("Appending {item:?} to {self:?}");
        if self.inner.len() == self.length {
            self.shrink();
        }
        self.inner[self.length] = item;
        self.length += 1;
    }

    pub fn remove(&mut self) -> T {
        let item = self.inner[self.length - 1].clone();
        self.inner[self.length - 1] = T::default();
        self.length -= 1;
        item
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        match idx < self.length {
            true => Some(&self.inner[idx]),
            _ => None,
        }
    }
}

mod tests {

    use super::*;

    #[test]
    fn test_array_list_append() {
        let mut arr = ArrayList::new(1);
        arr.append(1);
        assert_eq!(arr.length, 1);
    }

    #[test]
    fn test_array_list_get() {
        let mut arr = ArrayList::new(1);
        arr.append(1);
        assert_eq!(arr.get(0), Some(&1));
    }

    #[test]
    fn test_array_list_append_shrink() {
        let mut arr = ArrayList::new(4);
        arr.append(1);
        assert_eq!(arr.inner[0], 1);
        assert_eq!(arr.inner[1], 0);
        assert_eq!(arr.inner[2], 0);
        assert_eq!(arr.inner[3], 0);
        arr.append(2);
        assert_eq!(arr.inner[0], 1);
        assert_eq!(arr.inner[1], 2);
        assert_eq!(arr.inner[2], 0);
        assert_eq!(arr.inner[3], 0);
        arr.append(3);
        arr.append(4);
        arr.append(4);
        assert_eq!(arr.inner[4], 4);
        assert_eq!(arr.inner[5], 0);
        assert_eq!(arr.length, 5);
        assert_eq!(arr.capacity, 8);
    }

    #[test]
    fn test_array_list_remove_shrink() {
        let mut arr = ArrayList::new(4);
        arr.append(1);
        arr.append(2);
        arr.append(3);
        arr.append(-1);
        assert_eq!(4, arr.length);
        assert_eq!(-1, arr.remove());
        assert_eq!(4, arr.capacity);
        arr.append(1);
        arr.append(2);
        arr.append(3);
        assert_eq!(8, arr.capacity);
        assert_eq!(3, arr.remove());
        assert_eq!(2, arr.remove());
        assert_eq!(1, arr.remove());
        assert_eq!(vec![1, 2, 3, 0, 0, 0, 0, 0], arr.inner);
    }
}
