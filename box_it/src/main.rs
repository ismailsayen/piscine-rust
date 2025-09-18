use std::mem;

use box_it::*;

fn main() {
    let s = "5.5k 8.9k 32".to_owned();

    let boxed = parse_into_boxed(s);
    println!("Element value: {:?}", boxed[0]);
    println!("Element size: {:?} bytes", mem::size_of_val(&boxed[0]));

    let unboxed = into_unboxed(boxed);
    println!("Element value: {:?}", unboxed[0]);
    println!("Element size: {:?} bytes", mem::size_of_val(&unboxed[0]));

}