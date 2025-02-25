use data_structures::array::{
    Array,
    MemArray,
    InsertArray,
    ReplaceArray,
    RemoveArray,
    SearchArray,
    SortArray
};

#[test]
fn test_new_empty_array() {
    let array = Array::<u8>::new();
    assert_eq!(array.len(), 0);
}
#[test]
fn test_fixed_size_array() {
    let mut array =
        Array::<u8>::with_capacity(10).expect("Failed allocating memory for new array");
    // Cause another memory allocation by exceeding capacity
    for _ in 0..15 {
        assert!(array.insert(1).is_ok());
    }
    assert_eq!(array.memory_used(), 20);
    assert_eq!(array.len(), 15);
}
#[test]
#[should_panic]
fn test_insert_and_index() {
    let mut array = Array::<u8>::new();
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
    let mut array = Array::<u8>::new();
    // Test allocating up to 2^16 == 16384 bytes of memory
    for i in 0..16 {
        assert!(array.grow().is_ok());
        assert_eq!(array.len(), 0);
        assert_eq!(array.memory_used(), usize::pow(2, i));
    }
}
fn test_insert() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_pop() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_remove() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_replace() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_find() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_find_all() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_quick_sort() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_bubble_sort() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_merge_sort() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_selection_sort() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_insertion_sort() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_drop() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_display() {
    let a = Array::<u8>::new();
    todo!()
}
fn test_default() {
    let a = Array::<u8>::new();
    todo!()
}