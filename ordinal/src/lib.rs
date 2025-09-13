pub fn num_to_ordinal(x: u32) -> String {
    let mut n = x.to_string();
    match n.chars().nth(n.len() - 1).unwrap() {
        '1' => {
            if x % 100 != 11  {
                n.push_str("st");
                return n;
            } else {
                n.push_str("th");
                return n;
            }
        }
        '2' => {
            if x % 100 != 12 {
                n.push_str("nd");
                return n;
            } else {
                n.push_str("th");
                return n;
            }
        }
        '3' => {
            if x % 100 != 13  {
                n.push_str("rd");
                return n;
            } else {
                n.push_str("th");
                return n;
            }
        }
        _ => {
            n.push_str("th");
            return n;
        }
    }
}
