pub fn factorial(num: u64) -> u64 {
if num==0 || num == 1{
    return 1;
}

let mut res:u64=1;
let mut n:u64= num;
loop{
    if n==1{
        break;
    }
    res*=n;
    n-=1;
}
res
}