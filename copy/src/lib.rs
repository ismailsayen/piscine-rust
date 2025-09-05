pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let f = c as f64;
    (c, f.exp(), f.abs().ln())
}
pub fn str_function(a: String) -> (String, String) {
    let mut v: Vec<String> = Vec::new();
    for i in a.split(" ").collect::<Vec<&str>>() {
        let n2: f64 = i.parse().unwrap();
        v.push(n2.exp().to_string());
    }
    (a, v.join(" "))
}
pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>){
     let mut v: Vec<f64> = Vec::new();
     for &ch in b.iter() {
        let n: f64 = ch as f64;
        v.push(n.abs().ln());
    }
    (b, v)
}
