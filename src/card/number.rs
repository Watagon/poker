use std::ops::Add;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Number(pub u8);

impl From<&str> for Number {
    fn from(value: &str) -> Self {
        const NUMBERS: [&str; 13] = [
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ];
        let num = NUMBERS
            .iter()
            .position(|&a| a == value)
            .unwrap_or_else(|| panic!("Unrecognizable number: {value}")) as u8
            + 1;
        Self(num)
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let sn = if self.0 == 1 { 14 } else { self.0 };
        let on = if other.0 == 1 { 14 } else { other.0 };
        sn.partial_cmp(&on)
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut num = (self.0 + rhs.0) % 13;
        if num == 0 {
            num = 13;
        }
        Self(num)
    }
}

impl Add<u8> for Number {
    type Output = Number;

    fn add(self, rhs: u8) -> Self::Output {
        let rhs = Number(rhs);
        self + rhs
    }
}
