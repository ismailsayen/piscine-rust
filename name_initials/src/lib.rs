pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    names.iter().for_each(|&val| { 
        let words:Vec<_>=val.split(" ").collect();
        res.push(format!("{}. {}.",words[0].chars().next().unwrap(),words[1].chars().next().unwrap()));
    }
    );
    res
}
