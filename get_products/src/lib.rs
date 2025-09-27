pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut  res: Vec<usize> = vec![];
    for ele in arr.clone(){
        let prd=arr.clone().iter().filter(|e| **e!=ele).cloned().collect::<Vec<usize>>().iter().product::<usize>();
        res.push(prd);
    }
    res
}
