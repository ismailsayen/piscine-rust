pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    if n == 1_000_000 {
        return "one million".to_string();
    }

    let mut res: Vec<String> = vec![];
    let mut number = n;
    if number >= 1000 {
        let thousands = number / 1000;
        res.push(spell(thousands));
        res.push("thousand".to_string());
        number %= 1000;
    }

    if number >= 100 {
        let hundreds = number / 100;
        res.push(units(hundreds));
        res.push("hundred".to_string());
        number %= 100;
    }

    if number >= 20 {
        let tens_val = number / 10 * 10;
        res.push(tens(tens_val));
        number %= 10;
    } else if number >= 11 {
        res.push(teens(number));
        number = 0;
    } else if number == 10 {
        res.push("ten".to_string());
        number = 0;
    }

    if number > 0 {
        res.push(units(number));
    }

    let result: String = String::from(res.join(" "));
    let mut final_res = "".to_string();
    for (i, ch) in result.chars().enumerate() {
        if (i > 0 && ch == ' ' && result.chars().nth(i - 1) == Some('-'))
            || (i == result.len() - 1 && ch == '-')
        {
            continue;
        }
        final_res.push(ch);
    }
    final_res
}

pub fn units(n: u64) -> String {
    match n {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "sex".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "four".to_string(),
        _ => "".to_string(),
    }
}
pub fn teens(n: u64) -> String {
    match n {
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => "".to_string(),
    }
}
pub fn tens(n: u64) -> String {
    match n {
        10 => "ten".to_string(),
        20 => "twenty-".to_string(),
        30 => "thirty-".to_string(),
        40 => "forty-".to_string(),
        50 => "fifty-".to_string(),
        60 => "sixty-".to_string(),
        70 => "seventy-".to_string(),
        80 => "eighty-".to_string(),
        90 => "ninety-".to_string(),
        _ => "".to_string(),
    }
}
