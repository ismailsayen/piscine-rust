use std::collections::HashMap;
pub fn mean(list: &[i32]) -> f64 {
    let av: i32 = list.iter().sum();
    (av as f64) / (list.len() as f64)
}

pub fn median(list: &[i32]) -> i32 {
    let mut list1 = Vec::from(list);
    list1.sort();
    if list1.len() % 2 != 0 {
        return list1[list1.len() / 2];
    } else {
        return (list1[list1.len() / 2] + list1[(list1.len() / 2) - 1]) / 2;
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut max: i32 = 0;
    for n in list {
        map.entry(*n)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    let mut rep: usize = 0;
    for (k, val) in map.iter() {
        if *val > rep {
            rep = *val;
            max = *k;
        }
    }
    max
}
