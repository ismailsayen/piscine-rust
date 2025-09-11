use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        let phrase = String::from(name);
        Self {
            short_hand: "-".to_string() + &phrase.chars().nth(0).unwrap().to_string(),
            long_hand: "--".to_string() + &phrase,
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let callback = self.flags[input];
        match callback(argv[0], argv[1]){
            Ok(t)=>Ok(t),
            Err(_)=>Err("invalid float literal".to_string())
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let n1: f64 = a.parse()?;
    let n2: f64 = b.parse()?;
    let res = n1 / n2;
    Ok(res.to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let n1: f64 = a.parse()?;
    let n2: f64 = b.parse()?;
    let res = n1 % n2;
    Ok(res.to_string())
}
