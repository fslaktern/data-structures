mod array;
use array::{ArrayStructure, Array};

fn main() {
    let mut a: Array<u8> = ArrayStructure::new();
    a.insert(10);
    a.insert(20);
    a.insert(40);
    a.insert(40);
    a.insert(20);
    a.insert(20);
    a.insert(20);
    a.change(2, 30).expect("Couldn't replace value at index");
    a.replace(20, 30);
    a.pop();
    println!("{}", a.find_all(30).unwrap());
    println!("a={} (len {})", a, a.len());
}
