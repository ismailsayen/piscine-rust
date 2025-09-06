pub fn arrange_phrase(phrase: &str) -> String {
    let sp: Vec<&str> = phrase.split_whitespace().collect();
    let mut res: Vec<&str> = vec!["";sp.len()];
    
    for wrd in sp {
        for ch in wrd.chars() {
            if ch.is_digit(10) {
               if let Some(ind)= ch.to_digit(10){
                   res[(ind-1) as usize]=wrd;
               }
            }
        }
    }
    let r:Vec<String>=res.iter().map(|&val|{
        val.chars().filter(|&ch| !ch.is_numeric()).collect::<String>()
    }).collect();
    r.join(" ")
}
