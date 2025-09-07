use bigger::*;
use std::collections::HashMap;

fn main() {
    let hash: HashMap<&'static str, i32, _> = HashMap::from_iter([
        ("Daniel", 122),
        ("Ashley", 333),
        ("Katie", 334),
        ("Robert", 14),
    ]);

    println!(
        "The biggest of the elements in the HashMap is {}",
        bigger(hash)
    );
}