#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let lowercase: Vec<char> = ('a'..='z').collect();
    let uppercase: Vec<char> = ('A'..='Z').collect();
    let lowercasee_rev = lowercase.iter().rev().collect::<Vec<_>>();
    let uppercase_rev = uppercase.iter().rev().collect::<Vec<_>>();
    let mut res = String::new();
    for ch in original.chars() {
        if lowercase.contains(&ch) {
            let ind: usize = lowercase.iter().position(|&ele| ele == ch).unwrap();
            res.push(*lowercasee_rev[ind]);
            continue;
        } else if uppercase.contains(&ch) {
            let ind: usize = uppercase.iter().position(|&ele| ele == ch).unwrap();
            res.push(*uppercase_rev[ind]);
            continue;
        }
        res.push(ch);
    }
    if ciphered == res {
        return Ok(());
    }

    let e = CipherError { expected: res };
    Err(e)
}
