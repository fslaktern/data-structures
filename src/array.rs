use std::{
    alloc::{alloc, dealloc, Layout},
    clone::Clone,
    fmt,
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
    ptr,
};

pub struct Array<T> {
    ptr: *mut T,
    size: usize,
    capacity: usize,
}

/// Memory management of Array
pub trait MemArray<T> {
    fn new() -> Self;
    fn with_capacity(initial_capacity: usize) -> Result<Self, String>
    where
        Self: Sized;
    fn grow(&mut self) -> Result<(), String>;
    fn len(&self) -> usize;
    fn memory_used(&self) -> usize;
}

/// Adding items to an array
pub trait InsertArray<T> {
    fn insert(&mut self, item: T) -> Result<(), String>;
}

/// Removing items from an Array
pub trait RemoveArray<T> {
    fn pop(&mut self) -> Option<T>;
    fn remove(&mut self, index: usize) -> Result<T, String>;
}

/// Replacing instances of an item in an Array with a new item
pub trait ReplaceArray<T> {
    fn replace(&mut self, old_item: T, new_item: T);
}

/// Finding items or indices in the array
pub trait SearchArray<T> {
    fn find(&self, needle: T) -> Option<usize>;
    fn find_all(&self, needle: T) -> Option<Array<usize>>;
}

/// Ways to sort the array
pub trait SortArray<T> {
    fn quick_sort(&mut self);
    fn bubble_sort(&mut self);
    fn merge_sort(&mut self);
    fn selection_sort(&mut self);
    fn insertion_sort(&mut self);
}

impl<T> MemArray<T> for Array<T> {
    /// Initialize an empty array
    fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }
    /// Initialize an empty array with given capacity
    fn with_capacity(initial_capacity: usize) -> Result<Array<T>, String> {
        let layout = Layout::array::<T>(initial_capacity).expect("Invalid memory layout");
        let new_ptr = unsafe { alloc(layout) as *mut T };
        if new_ptr.is_null() {
            Err(format!(
                "Failed allocating {} bytes of memory",
                initial_capacity
            ))
        } else {
            Ok(Self {
                ptr: new_ptr,
                size: 0,
                capacity: initial_capacity,
            })
        }
    }
    /// Grow array by allocating a doubly-sized new array. Move old values to newly allocated array before deallocating the old array.
    fn grow(&mut self) -> Result<(), String> {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };

        let layout = Layout::array::<T>(new_capacity)
            .map_err(|err| format!("Invalid memory layout: {}", err))?;

        // Try allocating 1 or 2*self.capacity bytes of memory
        let new_ptr = unsafe { alloc(layout) as *mut T };
        // Null pointer returned means memory allocation failed
        if new_ptr.is_null() {
            return Err(format!(
                "Failed allocating {} bytes of memory for array",
                new_capacity
            ));
        }
        // If the Array already has elements, copy them to new memory location
        if self.capacity != 0 {
            unsafe {
                ptr::copy_nonoverlapping(self.ptr, new_ptr, self.size);
            }
            // Deallocate old memory
            let previous_layout = Layout::array::<T>(self.capacity)
                .map_err(|err| format!("Invalid memory layout: {}", err))?;

            unsafe {
                dealloc(self.ptr as *mut u8, previous_layout);
            }
        }
        self.ptr = new_ptr;
        self.capacity = new_capacity;
        Ok(())
    }
    /// Return pre-counted length of array. Normally O(n), but here O(1)
    fn len(&self) -> usize {
        self.size
    }
    /// Number of bytes used in memory
    fn memory_used(&self) -> usize {
        self.capacity
    }
}

impl<T> InsertArray<T> for Array<T> {
    /// Insert a value at the end. Double the allocated memory if there is no more space.
    fn insert(&mut self, value: T) -> Result<(), String> {
        if self.size == self.capacity {
            self.grow()?;
        }
        unsafe {
            let end = self.ptr.add(self.size);
            ptr::write(end, value)
        }
        self.size += 1;
        Ok(())
    }
}

impl<T: PartialEq + Clone> ReplaceArray<T> for Array<T> {
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
}

impl<T> RemoveArray<T> for Array<T> {
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
}

impl<T: PartialEq> SearchArray<T> for Array<T> {
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
    fn find_all(&self, needle: T) -> Option<Array<usize>> {
        let mut matches = Array::new();
        for i in 0..self.size {
            unsafe {
                if *self.ptr.add(i) == needle {
                    matches.insert(i).ok()?;
                }
            }
        }
        if matches.size == 0 {
            None
        } else {
            Some(matches)
        }
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;
    /// Return reference to value at index. O(1)
    /// NB: Index out of bounds will cause panic at runtime
    fn index(&self, index: usize) -> &Self::Output {
        if self.size <= index {
            panic!("Index out of bounds");
        } else {
            unsafe { &*self.ptr.add(index) }
        }
    }
}

impl<T> IndexMut<usize> for Array<T> {
    /// Return mutable reference to value at index. O(1)
    /// NB: Index out of bounds will cause panic at runtime
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.size <= index {
            panic!("Index out of bounds");
        } else {
            unsafe { &mut *self.ptr.add(index) }
        }
    }
}

impl<T: PartialEq> SortArray<T> for Array<T> {
    fn quick_sort(&mut self) {
        todo!()
    }
    fn merge_sort(&mut self) {
        todo!()
    }
    fn bubble_sort(&mut self) {
        todo!()
    }
    fn insertion_sort(&mut self) {
        todo!()
    }
    fn selection_sort(&mut self) {
        todo!()
    }
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Display for Array<T> {
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

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        // Proven to be safe to unwrap as memory layout has already been created in grow()
        let current_layout = Layout::array::<T>(self.capacity).unwrap();
        unsafe {
            dealloc(self.ptr as *mut u8, current_layout);
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new_empty_array() {
        let array: Array<u8> = Array::new();
        assert_eq!(array.size, 0);
    }
    #[test]
    fn test_fixed_size_array() {
        let mut array: Array<u8> =
            Array::with_capacity(10).expect("Failed allocating memory for new array");
        // Cause another memory allocation by exceeding capacity
        for _ in 0..15 {
            assert!(array.insert(1).is_ok());
        }
        assert_eq!(array.capacity, 20);
        assert_eq!(array.size, 15);
    }
    #[test]
    #[should_panic]
    fn test_insert_and_index() {
        let mut array: Array<u8> = Array::new();
        for x in 0..100 {
            assert!(array.insert(x).is_ok());
        }
        assert_eq!(array[0], 0);
        assert_eq!(array[99], 99);
        // Index out of bounds causing panic at runtime
        _ = array[100];
    }
    #[test]
    fn test_grow() {
        let mut array: Array<u8> = Array::new();
        // Test allocating up to 2^16 == 16384 bytes of memory
        for i in 0..16 {
            assert!(array.grow().is_ok());
            assert_eq!(array.size, 0);
            assert_eq!(array.capacity, usize::pow(2, i));
        }
    }
}
