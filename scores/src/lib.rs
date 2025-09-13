pub fn score(target: &str) -> u64 {
    let mut count: u64 = 0;
    for ch in target.chars(){
        let ch1=ch.to_ascii_uppercase();
        let point= match ch1 {
               'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T'=>1,
               'D' | 'G' =>2, 
               'B' | 'C' | 'M' | 'P' =>3, 
               'F' | 'H' | 'V' | 'W' | 'Y' =>4,  
                'K'=> 5,
                'J' | 'X'=> 8,
                'Q' | 'Z'=> 10,
                _=>0

            };
            count+=point;
    }
    count
}
