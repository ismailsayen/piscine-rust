pub fn search(array: &[i32], key: i32) -> Option<usize> {
    println!("{:?} {}", array, key);
    let mut ind: Option<usize> = None;
    for i in 0..array.len() {
        match array[i] {
            key => ind = Some(i),
            _ => continue,
        }
    }
    ind
}
