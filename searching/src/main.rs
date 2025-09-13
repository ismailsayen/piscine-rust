use searching::*;

fn main() {
    let ar = [1, 3, 4, 1, 8, 9, 11];
    let f = search(&ar ,1);
    println!(
        "the element 8 is last in the position {:?} in the array {:?}",
        f, ar
    );
}