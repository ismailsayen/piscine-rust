use convert_case::{Case, Casing};
pub fn expected_variable(str1: &str, str2: &str) -> Option<String> {
    if str1.to_ascii_lowercase() == str1.to_ascii_lowercase().to_case(Case::Camel)
        || str1.to_ascii_lowercase() == str1.to_ascii_lowercase().to_case(Case::Snake)
    {
        let n_change = edit_distance(&str1.to_ascii_lowercase(), &str2.to_ascii_lowercase());
        let len = str2.len();

        if len == 0 {
            return None;
        }

        let similarity = ((len as f64 - n_change as f64) / len as f64) * 100.0;
        if similarity >= 50.0 {
            return Some(format!("{}%", similarity.round()));
        } else {
            return None;
        }
    }

    None
}

pub fn edit_distance(source: &str, target: &str) -> usize {
    let a_chars: Vec<char> = source.chars().collect();
    let b_chars: Vec<char> = target.chars().collect();

    let len_a = a_chars.len();
    let len_b = b_chars.len();

    let mut dp = vec![vec![0; len_b + 1]; len_a + 1];

 
    for i in 0..=len_a {
        dp[i][0] = i;
    }
    for j in 0..=len_b {
        dp[0][j] = j;
    }

    for i in 1..=len_a {
        for j in 1..=len_b {
            if a_chars[i - 1] == b_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1]; 
            } else {
                dp[i][j] = 1 + std::cmp::min(dp[i - 1][j], std::cmp::min(dp[i][j - 1],dp[i - 1][j - 1], ));
            }
        }
    }
    for _row in &dp {
        
    }

    dp[len_a][len_b]
}