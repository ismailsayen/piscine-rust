use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut map: HashMap<&str, usize> = HashMap::new();
    for &word in words {
        if map.contains_key(&word) {
            let n = map.get(&word).unwrap();
            map.insert(&word, *n + 1);
            continue;
        }
        map.insert(&word, 1);
    }
    map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    
    frequency_count.keys().count()
}
