use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!(),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        let mut res = RomanNumber(vec![]);
        let mut nbr = value;

        loop {
            match nbr {
                0 => {
                    if res.0.is_empty() {
                        res.0.push(Nulla);
                    }
                    break;
                }
                1 => {
                    nbr -= 1;
                    res.0.push(I);
                }
                2 => {
                    nbr -= 2;
                    res.0.push(I);
                    res.0.push(I);
                }
                3 => {
                    nbr -= 3;
                    res.0.push(I);
                    res.0.push(I);
                    res.0.push(I);
                }
                4 => {
                    nbr -= 4;
                    res.0.push(I);
                    res.0.push(V);
                }
                5..9 => {
                    nbr -= 5;
                    res.0.push(V);
                }
                9 => {
                    nbr -= 9;
                    res.0.push(I);
                    res.0.push(X);
                }
                10..40 => {
                    nbr -= 10;
                    res.0.push(X);
                }
                40..50 => {
                    nbr -= 40;
                    res.0.push(X);
                    res.0.push(L);
                }
                50..90 => {
                    nbr -= 50;
                    res.0.push(L)
                }
                90..100 => {
                    nbr -= 90;
                    res.0.push(X);
                    res.0.push(C);
                }
                100..400 => {
                    nbr -= 100;
                    res.0.push(C)
                }
                400..500 => {
                    nbr -= 400;
                    res.0.push(C);
                    res.0.push(D);
                }
                500..900 => {
                    nbr -= 500;
                    res.0.push(D)
                }
                900..1000 => {
                    nbr -= 500;
                    res.0.push(C);
                    res.0.push(M);
                }
                _ => {
                    nbr -= 1000;
                    res.0.push(M)
                }
            }
        }
        res
    }
}