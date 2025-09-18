pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace()
        .map(|ele| {
            Box::new(if ele.contains('k') {
                let n = ele.to_string().replace("k", "").parse::<f64>().unwrap();

                (n * 1000.0) as u32
            } else {
                (ele.to_string().parse::<f64>().unwrap()) as u32
            })
        })
        .collect::<Vec<Box<u32>>>()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
   
    a.iter().map(|ele|{
         **ele
    }).collect()
}
