pub fn get_diamond(c: char) -> Vec<String> {
    let alph: Vec<char> = ('A'..='Z').collect();
    let mut result: Vec<String> = vec![];
    let mut res: String = String::new();
    let index: usize = alph
        .iter()
        .position(|ch| ch == &c.to_ascii_uppercase())
        .unwrap();
    for i in 0..=index {
        let spaces = " ".repeat(index - i);
        // repeat((i*2)-1)
        let mut middle_space = " ".to_string();
        if i != 0 {
            middle_space = " ".repeat((i * 2) - 1)
        } else {
            middle_space = " ".repeat(i * 2)
        }
        let mut lettre: String = String::from(alph[i].to_ascii_uppercase());
        lettre.push_str(&middle_space);
        if i > 0 {
            lettre.push(alph[i].to_ascii_uppercase());
        }
        res.push_str(&spaces);
        res.push_str(&lettre);
        res.push_str(&spaces);
        result.push(res.clone());
        res.clear();
    }
    for i in result.clone()[..result.len()-1].iter().rev(){
      result.push(i.to_string());
    }    
    result
}
