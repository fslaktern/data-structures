use std::alloc::{alloc, dealloc, Layout};
use std::clone::Clone;
// use std::mem;
use std::fmt;
use std::ptr;

// #[derive(fmt::Debug)]
pub struct Array<T> {
    ptr: *mut T,
    size: usize,
    capacity: usize,
}

pub trait ArrayStructure<T> {
    fn new() -> Self;
    fn grow(&mut self);
    fn insert(&mut self, item: T);
    fn change(&mut self, index: usize, new_item: T) -> Result<(), String>;
    fn replace(&mut self, old_item: T, new_item: T);
    fn pop(&mut self) -> Option<T>;
    fn remove(&mut self, index: usize) -> Result<T, String>;
    fn find(&self, needle: T) -> Option<usize>;
    fn find_all(&self, needle: T) -> Option<Array<usize>>;
    fn index(&self, index: usize) -> Option<&T>;
    fn len(&self) -> usize;
}

impl<T: PartialEq + Clone> ArrayStructure<T> for Array<T> {
    /// Initialize an empty array
    fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }
    /// Grow array by allocating a doubly-sized new array. Move old values to newly allocated array before deallocating the old array.
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };
        let layout =
            Layout::array::<T>(new_capacity).expect("Failed creating layout for new array");
        let new_ptr = if self.capacity == 0 {
            unsafe { alloc(layout) as *mut T }
        } else {
            unsafe {
                let new_ptr = alloc(layout) as *mut T;
                ptr::copy_nonoverlapping(self.ptr, new_ptr, self.size);

                // Create layout to give dealloc something to free
                let old_layout = Layout::array::<T>(self.capacity)
                    .expect("Failed creating layout for old memory");
                dealloc(self.ptr as *mut u8, old_layout);

                new_ptr
            }
        };
        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
    /// Insert a value at the end. Double the allocated memory if there is no more space.
    fn insert(&mut self, value: T) {
        if self.size == self.capacity {
            self.grow();
        }
        unsafe {
            let end = self.ptr.add(self.size);
            ptr::write(end, value)
        }
        self.size += 1;
    }
    /// Change a value at an index
    fn change(&mut self, index: usize, value: T) -> Result<(), String> {
        if self.size < index {
            return Err("Index out of bounds".into());
        }
        unsafe {
            ptr::write(self.ptr.add(index), value);
        }
        Ok(())
    }
    /// Replace all instances of a value in the array
    fn replace(&mut self, old_item: T, new_item: T) {
        for i in 0..self.size {
            unsafe {
                if *self.ptr.add(i) == old_item {
                    ptr::write(self.ptr.add(i), new_item.clone());
                }
            }
        }
    }
    /// Remove last element and return it. O(1)
    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;

        unsafe {
            let end = self.ptr.add(self.size);
            Some(ptr::read(end))
        }
    }
    /// Remove element at given index and return it. O(1)..O(n)
    fn remove(&mut self, index: usize) -> Result<T, String> {
        if self.size < index {
            return Err("Index out of bounds".into());
        } else if self.size == index - 1 {
            // Special case: If removing the last element, use pop() for O(1) complexity
            return Ok(self.pop().unwrap());
        }
        unsafe {
            let removed = ptr::read(self.ptr.add(index));

            // Shift elements left: from index + 1 to the end
            ptr::copy(
                // Source: Everything after index
                self.ptr.add(index + 1),
                // Destination: To the specified index
                self.ptr.add(index),
                // Elements from source to shift: Everything after the element at index
                self.size - index - 1,
            );
            self.size -= 1;
            Ok(removed)
        }
    }
    /// Loop through array to find the first needle. O(1)..O(n)
    fn find(&self, needle: T) -> Option<usize>
    where
        T: PartialEq,
    {
        for i in 0..self.size {
            unsafe {
                if *self.ptr.add(i) == needle {
                    return Some(i);
                }
            }
        }
        None
    }
    /// Loop through entire array to find all matching needles. O(n)
    fn find_all(&self, needle: T) -> Option<Array<usize>>
    where
        T: PartialEq
    {
        let mut matches = Array::new();
        for i in 0..self.size {
            unsafe {
                if *self.ptr.add(i) == needle {
                    matches.insert(i);
                }
            }
        }
        match matches.size {
            0 => None,
            _ => Some(matches)
        }
        
    }
    /// Return value at index. O(1)
    fn index(&self, index: usize) -> Option<&T> {
        if self.size < index {
            return None;
        }
        unsafe { Some(&*self.ptr.add(index)) }
    }
    /// Return length of array. O(1)
    fn len(&self) -> usize {
        self.size
    }
}

impl<T: fmt::Debug> fmt::Display for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.size {
            if i != 0 {
                write!(f, ", ")?;
            }
            unsafe {
                write!(f, "{:?}", *self.ptr.add(i))?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

