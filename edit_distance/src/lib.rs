use std::cmp::min;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let s_len = source.chars().count() + 1;
    let t_len = target.chars().count() + 1;

    let mut table = vec![vec![0; t_len]; s_len];

    for (y, row) in table.iter_mut().enumerate() {
        for (x, col) in row.iter_mut().enumerate() {
            if y == 0 {
                *col = x as i32;
            }
        }
        row[0] = y as i32;
    }

    let mut clone_table = table.clone();

    for (y, row) in table.iter_mut().enumerate() {
        if y > 0 {
            for (x, _) in row.iter_mut().enumerate() {
                if x > 0 {
                    let v1 = clone_table[y][x - 1];
                    let v2 = clone_table[y - 1][x];
                    let v3 = clone_table[y - 1][x - 1];
                    if source.chars().nth(y - 1) != target.chars().nth(x - 1) {
                        clone_table[y][x] = min(v1, min(v2, v3)) + 1;
                    } else {
                        clone_table[y][x] = min(v1, min(v2, v3));
                    }
                }
            }
        }
    }

    clone_table[clone_table.len() - 1][clone_table[0].len() - 1] as usize
}
