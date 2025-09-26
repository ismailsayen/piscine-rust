pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    println!("{}=> {}",prefix,s);
    if prefix.chars().count() > s.chars().count() {
        return None;
    }
    let mut pr = "".to_string();
    for (i, ch) in s.chars().enumerate() {
        if i ==  prefix.chars().count() {
            break;
        }
        pr.push(ch);
    }
    
    if prefix != pr {
        return None;
    }
   s.strip_prefix(prefix)
}
