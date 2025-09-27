pub fn first_fifty_even_square() -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut nbr: i32 = 1;
    let is_even = |n: i32| -> bool {
        if n % 2 == 0 {
            return true;
        }
        false
    };
    loop {
        let square: i32 = nbr.pow(2);
        if is_even(square) {
            res.push(square)
        }
        if res.len() == 50 {
            break;
        }
        nbr += 1;
    }
    res
}
