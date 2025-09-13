pub fn talking(text: &str) -> &str {
   println!("{:?}",text);
    if text.trim().chars().count() == 0 {
        return "Just say something!";
    }
    match text.trim().chars().last().unwrap() {
        '!' => {
            if is_yelling(text) {
                return "There is no need to yell, calm down!";
            }
            return "Interesting";
        }
        '?' => {
            if is_yelling(text) {
                return "Quiet, I am thinking!";
            }
            return "Sure.";
        }
        _ => "Interesting",
    }
}

pub fn is_yelling(text: &str) -> bool {
    for ch in text.chars() {
        if (ch.is_ascii_alphanumeric() && ch.is_ascii_lowercase()) || ch.is_numeric() {
            return false;
        }
    }

    true
}
