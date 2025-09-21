#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;
impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sign = "+".to_string();
        let mut blood = String::new();
        if s.contains('-') {
            sign = "-".to_string();
            blood = s.replace("-", "");
        } else if s.contains('+') {
            sign = "+".to_string();
            blood = s.replace("+", "");
        }
        Ok(Self {
            antigen: Antigen::from_str(&blood)?,
            rh_factor: RhFactor::from_str(&sign)?,
        })
    }
}
impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "AB" => Ok(Self::AB),
            "B" => Ok(Self::B),
            "O" => Ok(Self::O),
            _ => Err("Not FOUND".to_string()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Positive),
            "-" => Ok(Self::Negative),
            _ => Err("notFOund".to_string()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.antigen.cmp(&other.antigen)
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{:?}{}", self.antigen, sign)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor == RhFactor::Negative && other.rh_factor == RhFactor::Positive {
            return false;
        }
        match (&self.antigen, &other.antigen) {
            (Antigen::AB, _) => true,
            (_, Antigen::O) => true,
            (Antigen::A, Antigen::A) => true,
            (Antigen::B, Antigen::B) => true,
            _ => false,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut res: Vec<Self> = vec![];
        let blood = [Antigen::A, Antigen::B, Antigen::O, Antigen::AB];
        let factor = [RhFactor::Positive, RhFactor::Negative];
        for b in &blood {
            for r in &factor {
                let blood_t = BloodType {
                    antigen: b.clone(),
                    rh_factor: r.clone(),
                };
                if self.can_receive_from(&blood_t) {
                    res.push(blood_t);
                }
            }
        }
        res
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut res: Vec<Self> = vec![];
        let blood = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        let factor = [RhFactor::Positive, RhFactor::Negative];
        for b in &blood {
            for r in &factor {
                let blood_t = BloodType {
                    antigen: b.clone(),
                    rh_factor: r.clone(),
                };
                if blood_t.can_receive_from(self) {
                    res.push(blood_t);
                }
            }
        }
        res
    }
}
