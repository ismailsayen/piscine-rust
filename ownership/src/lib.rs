pub fn first_subword(mut s: String) -> String {
    let mut res = String::new();
    let mut first = true;
    for w in s.chars() {
        if w.is_uppercase() && first {
            first = false;
            res.push(w);
            continue;
        }
        if !w.is_uppercase() && w != '_' {
            res.push(w);
             first = false;
        }else {
            break;
        }
    }
    res
}
