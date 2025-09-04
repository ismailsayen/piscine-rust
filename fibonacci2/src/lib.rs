pub fn fibonacci(n: u32) -> u32 {
    let mut v =vec![0,1];
    for i in 0..=n as usize{
        if i >= 1{
            let  a =v[i];
            let  b=v[i-1];
            let  res=a+b;
            v.push(res)
        } 
    }
    v[n  as usize] 
}