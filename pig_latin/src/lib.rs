pub fn pig_latin(text: &str) -> String {
    let mut res = String::new();
    if is_vowel(&text.chars().next().unwrap()) {
        res.push_str(text);
        res.push_str("ay");
        return res;
    } else {
        if text.len() >= 3 {
            if text.chars().nth(1).unwrap() == 'q' && text.chars().nth(2).unwrap() == 'u' {
                res.push_str(&text[3..]);
                res.push_str(&text[..3]);
                res.push_str("ay");
                return res;
            }
            let mut bfr_vwl = String::new();
            for (index, ch) in text.chars().enumerate() {
                if is_vowel(&ch) {
                    res.push_str(&text[index..]);
                    break;
                }
                bfr_vwl.push(ch);
            }
            res.push_str(&bfr_vwl);
            res.push_str("ay");
        }
    }
    res
}

pub fn is_vowel(ch: &char) -> bool {
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
