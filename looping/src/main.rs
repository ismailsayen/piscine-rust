use std::io;
fn main() {
    let mut n_try: u32 = 0;
    loop {
        let mut input: String = String::new();
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        n_try += 1;
        io::stdin().read_line(&mut input).unwrap();
        if input.trim_matches(|c| c == '\n') == "The letter e" {
            println!("Number of trials: {}", n_try);
            break;
        }
    }
}
