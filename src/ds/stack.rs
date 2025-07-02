/* Stack
* Lifo - last in first out
* insert at the top of the Stack
* take from the top of the Stack
*
* Queue
* Fifo - first in last out
* */

pub mod linked_list;
pub use linked_list::*;

pub struct Queue<T> {
    head: LinkedList<T>,
}
