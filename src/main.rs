mod array;
use array::{Array, MemArray, InsertArray};

fn main() {
    let mut a = Array::<&str>::new();
    a.insert("h").unwrap();
    println!("{}", a);
    a.insert("e").unwrap();
    println!("{}", a);
    a[0] = "xxxxxxxxxxxx";
    println!("{}", a);
}
