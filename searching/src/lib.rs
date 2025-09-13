pub fn search(array: &[i32], key: i32) -> Option<usize> {
    println!("{:?} {}",array,key);
    let mut ind: Option<usize> = None;
    for i in (0..array.len()).rev() {
        ind = if array[i as usize] == key {
            Some(i as usize)
        }else{
            continue;
        }
    }
    ind
}
