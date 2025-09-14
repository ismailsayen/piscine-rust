pub fn rotate(input: &str, key: i8) -> String {
    let arr = ('a'..='z').collect::<Vec<char>>();
    let mut res = String::new();
    for (_, ch) in input.chars().enumerate() {
        if !ch.is_ascii_alphabetic() {
            res.push(ch);
            continue;
        }
        let rot = arr
            .iter()
            .position(|&c| c == ch.to_ascii_lowercase())
            .unwrap() as i8;
        let mut new_index: i8 = rot + key;
        if ch.is_ascii_lowercase() {
            if new_index >= arr.len() as i8 {
                new_index -= arr.len() as i8;
                res.push(arr[new_index as usize]);
            } else if new_index < 0 {
                new_index = arr.len() as i8 + new_index;
                res.push(arr[new_index as usize]);
            } else {
                res.push(arr[new_index as usize]);
            }
        } else {
            if new_index >= arr.len() as i8 {
                new_index -= arr.len() as i8;
                res.push(arr[new_index as usize].to_ascii_uppercase());
            } else if new_index < 0 {
                new_index = arr.len() as i8 + new_index;
                res.push(arr[new_index as usize].to_ascii_uppercase());
            } else {
                res.push(arr[new_index as usize].to_ascii_uppercase());
            }
        }
    }
    res
}
