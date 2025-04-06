use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr;
use std::{
    fmt,
    fmt::{Debug, Display},
};

enum LinkedListType {
    SinglyLinkedList,
    DoublyLinkedList,
}

pub struct DoublyLinkedList<T> {
    head: *mut DoublyLinkedListNode<T>,
    tail: *mut DoublyLinkedListNode<T>,
}

pub struct DoublyLinkedListNode<T> {
    // First node does not have a previous node
    previous: *mut DoublyLinkedListNode<T>,
    data: T,
    // Last node does not have a last node
    next: *mut DoublyLinkedListNode<T>,
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
    fn replace(&mut self, old_item: T, new_item: T);
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
            size += unsafe { mem::size_of_val(&(*node).previous) };
            size += unsafe { mem::size_of_val(&(*node).data) };
            size += unsafe { mem::size_of_val(&(*node).next) };
            node = unsafe { (*node).next };
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
            self.tail = new_node;
        } else {
            unsafe {
                (*new_node).previous = self.tail;
                (*self.tail).next = new_node;
            }
            self.tail = new_node;
        }
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
                        // As a result, `(*new_node).next` will never be null, and can safely be dereference to access `(*(*new_node).next).previous`

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
                drop(Box::from_raw(node));
            };
        }

        Ok(())
    }
    fn remove_at(&mut self, index: usize) -> Result<T, String> {
        todo!()
    }
    fn replace(&mut self, old_item: T, new_item: T) {
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
        if !self.head.is_null() {
            let mut node = self.head;
            while !node.is_null() {
                write!(f, "{:?}", unsafe { &(*node).data })?;
                if unsafe { (*node).next.is_null() } {
                    break;
                }
                write!(f, ", ")?;
                node = unsafe { (*node).next };
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        let mut node = self.head;
        let mut next;

        while !node.is_null() {
            next = unsafe { (*node).next };
            unsafe {
                drop(Box::from_raw(node));
            }
            node = next;
        }
    }
}

impl<T> Index<isize> for DoublyLinkedList<T> {
    type Output = DoublyLinkedListNode<T>;

    fn index(&self, mut index: isize) -> &Self::Output {
        if index < 0 {
            index += self.len() as isize;
            if index < 0 {
                panic!("Index {} is out of bounds", index);
            }
        }
        let mut current_index = 0;
        let mut node = self.head;
        while !node.is_null() {
            if current_index == index {
                return unsafe { &*node };
            }
            current_index += 1;
            node = unsafe { (*node).next };
        }
        panic!("Index {} is out of bounds", index);
    }
}

impl<T> IndexMut<isize> for DoublyLinkedList<T> {
    fn index_mut(&mut self, mut index: isize) -> &mut DoublyLinkedListNode<T> {
        if index < 0 {
            index += self.len() as isize;
            if index < 0 {
                panic!("Index {} is out of bounds", index);
            }
        }

        let mut current_index = 0;
        let mut node = self.head;
        while !node.is_null() {
            if current_index == index {
                return unsafe { &mut *node };
            }
            current_index += 1;
            node = unsafe { (*node).next };
        }
        panic!("Index {} is out of bounds", index);
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
