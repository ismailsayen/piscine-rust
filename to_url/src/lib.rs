pub fn to_url(s: &str) -> String {
    let sen = String::from(s);
    sen.replace(" ", "%20")
}
