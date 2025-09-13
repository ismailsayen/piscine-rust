use pangram::*;

fn main() {
    println!(
        "{}",
        is_pangram("the quick brown fox jumps over the lazy dog!")
    );
    println!("{}", is_pangram("Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich"));
}