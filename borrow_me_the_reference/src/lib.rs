pub fn delete_and_backspace(s: &mut String) {
    let  v: Vec<char> = s.chars().collect();
    let mut res: String = String::new();
    let mut f: usize = 0;
    for  i in 0.. v.len() {
            if v[i]=='-'  {
                if !res.is_empty() {
                    res.pop();
                   
                }
            }
            if v[i]=='+' {
                f += 1;
              
            }
            if v[i]!='-' && v[i]!='+'{
                if f > 0 {
                    f-=1
                } else {
                    res.push(v[i]);
                }
    }
    *s = res.clone();
}
}

pub fn do_operations(v: &mut [String]) {
    let mut sign: &str = "+";
    for val in v.iter_mut() {
        let mut nums: Vec<&str> = val.split("+").collect();
        if nums.len() != 2 {
            nums = val.split("-").collect();
            sign = "-";
        }
        let n1: i64 = nums[0].parse().unwrap();
        let n2: i64 = nums[1].parse().unwrap();
        if sign == "+" {
            let r = n1 + n2;
            *val = r.to_string();
        }else{
            let r = n1 - n2;
            *val = r.to_string();
        }
    }
}
