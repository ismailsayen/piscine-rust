pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut ind: Option<usize> = None;
    for i in (0..array.len()).rev() {
        if array[i as usize] == key {
            ind = Some(i as usize);
        }
    }
    ind
}
