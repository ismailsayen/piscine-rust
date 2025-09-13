pub fn search(array: &[i32], key: i32) -> Option<usize> {
    println!("{:?} {}", array, key);
    let mut ind: Option<usize> = None;
    for i in 0..array.len() {
       let  a  = match array[i] {
            key => i,
            _ => continue,
        };
        ind=Some(a);
    }
    ind
}
