pub fn capitalize_first(input: &str) -> String {
    let mut res: String = "".to_string();
    let mut first_letter: bool = false;
    for ch in input.chars() {
        if ch.is_alphabetic() && !first_letter {
            let up = ch.to_ascii_uppercase();
            res.push(up);
            first_letter = true;
            continue;
        }
        res.push(ch);
    }
    res
}

pub fn title_case(input: &str) -> String {
    let mut res: String = "".to_string();
    let mut first_letter: bool = true;
    for ch in input.chars() {
        if !ch.is_alphanumeric() {
            first_letter = true;
        }
        if ch.is_alphabetic() && first_letter {
            let up = ch.to_ascii_uppercase();
            res.push(up);
            first_letter = false;
            continue;
        }
        res.push(ch);
    }
    res
}

pub fn change_case(input: &str) -> String {
    let mut res: String = "".to_string();
    for ch in input.chars() {
        if ch.is_alphanumeric() && ch.is_ascii_lowercase() {
            let up = ch.to_ascii_uppercase();
            res.push(up);
        } else if ch.is_alphanumeric() && ch.is_ascii_uppercase() {
            let up = ch.to_ascii_lowercase();
            res.push(up);
        } else {
            res.push(ch);
        }
    }
    res
}
