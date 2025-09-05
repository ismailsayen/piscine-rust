pub fn delete_and_backspace(s: &mut String) {
    let mut v: Vec<char> = s.chars().collect();
    let mut res: String = String::new();
    let mut f: usize = 0;
    let mut i: usize = 0;

    // let mut b: usize = 0;
    while i < v.len() {
        let ch = v[i];
        match v[i] {
            '-' => {
                if !res.is_empty() {
                    res.pop();
                    i += 1;
                }
            }
            '+' => {
                f += 1;
                i += 1;
            }
            _ => {
                if f != 0 {
                    i += f;
                    f = 0;
                    continue;
                } else {
                    i += 1;
                }
                res.push(ch);
            }
        }
    }
    *s = res
}

pub fn do_operations(v: &mut [String]) {
    let mut sign: &str = "+";
    for val in v {
        let mut nums: Vec<&str> = vec![];
        nums = val.split("+").collect();
        if nums.len() != 2 {
            nums = val.split("-").collect();
            sign = "-";
        }
        if sign == "+" {
            let n1:i64= nums[0].parse().unwrap();
            let n2:i64= nums[1].parse().unwrap();
            let r = n1 + n2;
            *val = r.to_string();
        }
        println!("{:?} {sign}", nums);
        
    }
}
