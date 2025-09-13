pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut ind:Option<usize>=None;
    for i in 0..array.len(){
        match key {
            8i32=>ind=Some(i),
            _=>continue,
        }
    }
    ind
}