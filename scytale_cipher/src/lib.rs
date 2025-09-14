pub fn scytale_cipher(message: &str, i: usize) -> String {
    println!("{:?} {:?}", message,i);
    let mut cipher: Vec<Vec<char>> = vec![];
    let mut word: Vec<char> = vec![];
    let mut res = String::new();
    for (index, ch) in message.chars().enumerate() {
        word.push(ch);
        if word.len() < i && index + 1 == message.len() {
            while word.len() < i {
                word.push(' ');
            }
            cipher.push(word.clone());
            break;
        }
        if word.len() == i {
            cipher.push(word.clone());
            word.clear();
        }
    }
    println!("{:?}", cipher);
    for id in 0..i {
        for sl in 0..cipher.len() {
            res.push(cipher[sl][id]);
           
        }
    }

    (&res.trim()).to_string()
}
