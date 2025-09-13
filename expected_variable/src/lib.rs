use convert_case::{Case, Casing};
use edit_distance::*;
pub fn expected_variable(str1: &str, str2: &str) -> Option<String> {
    if str1.to_lowercase() == str1.to_lowercase().to_case(Case::Camel)
        || str1.to_lowercase() == str1.to_lowercase().to_case(Case::Snake)
    {
        let n_change = edit_distance(&str1.to_lowercase(), &str2.to_lowercase());
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
