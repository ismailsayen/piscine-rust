use scytale_cipher::scytale_cipher;

fn main() {
    println!("\"scytale Code\" size=6 -> {:?}", scytale_cipher("" ,13));
    println!("\"scytale Code\" size=8 -> {:?}", scytale_cipher("scytale Code", 8));
}