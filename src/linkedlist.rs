use std::fmt::{self, Debug, Display};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr;

#[repr(C)]
pub struct DoublyLinkedList<T> {
    head: *mut DoublyLinkedListNode<T>,
    tail: *mut DoublyLinkedListNode<T>,
}

pub struct DoublyLinkedListNode<T> {
    pub previous: *mut DoublyLinkedListNode<T>,
    data: T,
    pub next: *mut DoublyLinkedListNode<T>,
}

pub struct DoublyLinkedListIterator<'a, T> {
    current: *mut DoublyLinkedListNode<T>,
    phantom: PhantomData<&'a T>,
}

pub trait LinkedList<T> {
    fn new() -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn size_of_value(&self) -> usize;
    fn append(&mut self, data: T) -> Result<(), String>;
    fn insert_at(&mut self, data: T, index: usize) -> Result<(), String>;
    fn pop(&mut self) -> Result<(), String>;
    fn remove_at(&mut self, index: usize) -> Result<T, String>;
    fn replace_all(&mut self, old_item: T, new_item: T);
    fn swap(&mut self, index_a: usize, index_b: usize);
    fn find_one(&self, needle: T) -> Option<usize>;
    fn find_all(&self, needle: T) -> Option<Vec<usize>>;
}

pub trait Sorting<T> {
    fn bubble_sort(&mut self);
    fn insertion_sort(&mut self);
    fn quick_sort(&mut self);
}

impl<T> LinkedList<T> for DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }
    // Count every item one by one until the current item is a null pointer
    fn len(&self) -> usize {
        let mut count = 0;
        let mut node = self.head;
        while !node.is_null() {
            count += 1;
            node = unsafe { (*node).next };
        }
        count
    }

    fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    fn size_of_value(&self) -> usize {
        let mut size = 0;
        let mut node = self.head;
        while !node.is_null() {
            // Count pointers and node data
            unsafe {
                size += mem::size_of_val(&(*node).previous);
                size += mem::size_of_val(&(*node).data);
                size += mem::size_of_val(&(*node).next);
                node = (*node).next;
            }
        }
        size
    }

    fn append(&mut self, data: T) -> Result<(), String> {
        let new_node = Box::into_raw(Box::new(DoublyLinkedListNode {
            previous: ptr::null_mut(),
            data,
            next: ptr::null_mut(),
        }));

        if self.head.is_null() {
            self.head = new_node;
        } else {
            unsafe {
                (*new_node).previous = self.tail;
                (*self.tail).next = new_node;
            }
        }
        self.tail = new_node;
        Ok(())
    }

    fn insert_at(&mut self, data: T, index: usize) -> Result<(), String> {
        let new_node = Box::into_raw(Box::new(DoublyLinkedListNode {
            previous: ptr::null_mut(),
            data,
            next: ptr::null_mut(),
        }));

        let mut current_index = 0;
        let mut node = self.head;
        while !node.is_null() {
            if current_index == index {
                // If we want to insert at the start of the list, we need to update `self.head`
                if index == 0 {
                    unsafe {
                        (*new_node).next = self.head;
                        (*self.head).previous = new_node;
                        // `(*new_node).previous` will stay a null pointer until something slides in before it
                    }
                    self.head = new_node;
                } else {
                    unsafe {
                        // Update new_node to point to previous node at [i-1] and next node at [i]
                        (*new_node).previous = (*node).previous;
                        (*new_node).next = node;

                        // This function will not allow an item to be put at the end of the list as it exits one step before this can happen
                        // As a result, `(*new_node).next` will never be null, and can safely be dereferenced to access `(*(*new_node).next).previous`

                        // Update surrounding nodes to point to new_node
                        (*(*new_node).previous).next = new_node;
                        (*(*new_node).next).previous = new_node;
                    }
                }
                return Ok(());
            }
            node = unsafe { (*node).next };
            current_index += 1;
        }
        Err("Index is out of bounds".to_string())
    }

    /// Remove last item in the list. O(1)
    fn pop(&mut self) -> Result<(), String> {
        if self.tail.is_null() {
            return Err("List is already empty".to_string());
        }
        let node = self.tail;

        if unsafe { (*node).previous.is_null() } {
            self.head = ptr::null_mut();
            self.tail = ptr::null_mut();
        } else {
            let previous = unsafe { (*node).previous };
            self.tail = previous;
            unsafe {
                (*previous).next = ptr::null_mut();
            };
        }
        unsafe {
            drop(Box::from_raw(node));
        };

        Ok(())
    }
    fn remove_at(&mut self, index: usize) -> Result<T, String> {
        todo!()
    }
    fn replace_all(&mut self, old_item: T, new_item: T) {
        todo!()
    }
    fn swap(&mut self, index_a: usize, index_b: usize) {
        todo!()
    }
    fn find_one(&self, needle: T) -> Option<usize> {
        todo!()
    }
    fn find_all(&self, needle: T) -> Option<Vec<usize>> {
        todo!()
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut node = self.head;
        let mut first = true;

        while !node.is_null() {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            unsafe {
                write!(f, "{:?}", &(*node).data)?;
                node = (*node).next;
            }
        }
        write!(f, "]")
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        let mut node = self.head;
        while !node.is_null() {
            unsafe {
                let next = (*node).next;
                // Node is deliberately boxed to be freed when out of scope
                #[allow(unused)]
                Box::from_raw(node);
                node = next;
            }
        }
    }
}

impl<T> Index<usize> for DoublyLinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut i = 0;
        let mut node = self.head;
        while !node.is_null() {
            if i == index {
                return unsafe { &(*node).data };
            }
            i += 1;
            unsafe { node = (*node).next; }
        }
        panic!("Index {} is out of bounds", index);
    }
}

impl<T> IndexMut<usize> for DoublyLinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut i = 0;
        let mut node = self.head;
        while !node.is_null() {
            if i == index {
                return unsafe { &mut (*node).data };
            }
            i += 1;
            unsafe {
                node = (*node).next;
            }
        }
        panic!("Index {} is out of bounds", index);
    }
}

impl<'a, T> IntoIterator for &'a DoublyLinkedList<T> {
    type Item = &'a T;
    type IntoIter = DoublyLinkedListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        DoublyLinkedListIterator {
            current: self.head,
            phantom: PhantomData
        }
    }
}

impl<'a, T> Iterator for DoublyLinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }
        unsafe {
            let node_ref = &*self.current;
            self.current = node_ref.next;
            return Some(&node_ref.data);
        }
    }
}

impl<T> Sorting<T> for DoublyLinkedList<T> {
    fn bubble_sort(&mut self) {
        todo!()
    }
    fn quick_sort(&mut self) {
        todo!()
    }
    fn insertion_sort(&mut self) {
        todo!()
    }
}
