use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    let mut chars_in_s: HashMap<char, u32> = HashMap::new();
    for ch in s.chars(){
        if ch.is_ascii_alphabetic(){
            chars_in_s.entry(ch).and_modify(|count|{*count+=1}).or_insert(1);
        }else{
            continue;
        }
    }
    if chars_in_s.len()==26{
        return true;
    }
    false
}
