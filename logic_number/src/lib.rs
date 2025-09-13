pub fn number_logic(num: u32) -> bool {
    let s: String = num.to_string();
    let mut count: u32 = 0;
    for i in s.chars() {
        let parse=i.to_digit(10).unwrap();
        count += parse.pow(s.len() as u32);
    }
    if count == num{
        return true;
    }
    false
}
