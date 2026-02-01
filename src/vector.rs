use std::{
    alloc::{self, alloc, Layout},
    isize, mem,
    ptr::NonNull,
};

pub struct MyVec<T> {
    ptr: NonNull<T>,
    capacity: usize,
    len: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec {
            ptr: NonNull::dangling(),
            capacity: 0,
            len: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(
            std::mem::size_of::<T>(),
            0,
            "Zero-sized types are not supported"
        );
        if std::mem::size_of::<T>() == 0 {
            panic!("Zero-sized types are not supported");
        }

        if self.capacity == 0 {
            let layout = Layout::array::<T>(4).expect("Could not allocate");
            // SAFETY: the layout is hardcoded to be 4* size_of::<T>()
            // size_of<T> is > 0
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Could not allocate");
            // SAFETY: ptr is non null
            // SAFETY: we just allocated this memory for this item
            unsafe { ptr.as_ptr().write(item) };
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            let offset = self
                .len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Cannot reach memory locatity");
            assert!(offset < isize::MAX as usize, "Wrapper isize");
            // add will calculate the correct position
            // add skip len items
            // SAFETY: we have allocated enough memory for len + 1 items
            unsafe { self.ptr.as_ptr().add(self.len).write(item) };
            self.len += 1;
        } else {
            debug_assert!(self.len == self.capacity);
            let new_capacity = self.capacity.checked_mul(2).expect("Capacity wrapped");
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(size % align).expect("Can not allocate");
            let ptr = unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Error realloc: could not realloc");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            };
            self.ptr = ptr;
            self.len += 1;
            self.capacity = new_capacity;
        }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { &*self.ptr.as_ptr().add(index) })
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.capacity == 0 {
            return; // Nothing to deallocate
        }
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout = alloc::Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity,
                std::mem::align_of::<T>(),
            );
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MyVec;

    #[test]
    fn it_works() {
        let mut vec = MyVec::<usize>::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 0);
        vec.push(1);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.capacity(), 4);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);
        assert_eq!(vec.len(), 5);
        assert_eq!(vec.capacity(), 8);

        let value = Some(&5);
        assert_eq!(vec.get(4), value);
    }
}
