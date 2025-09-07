use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    create_map(s1) == create_map(s2)
}

pub fn create_map(sen: &str) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();
    for ch in sen.chars() {
        map.entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    map
}
