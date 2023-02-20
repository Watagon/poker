use std::{collections::HashMap, ops::Add};

use itertools::Itertools;
// use itertools::Itertools;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands_strs: &[&'a str]) -> Vec<&'a str> {
    // unimplemented!("Out of {hands:?}, which hand wins?")
    let hands: Vec<Hand> = hands_strs.iter().map(|&a| a.into()).collect();
    let res = hands.iter().map(|a| a.hand()).collect::<Vec<_>>();
    let res = res
        .iter()
        .enumerate()
        .sorted_by(|a, b| {
            a.1.partial_cmp(b.1)
                .unwrap_or_else(|| panic!("Err: partial_cmp: a: {a:?}, b: {b:?}"))
        })
        .rev();
    let mut res = res.peekable();
    let max = res.peek().unwrap().1;
    let mut res = res.filter(|a| a.1.eq(max)).collect::<Vec<_>>();
    res.sort_by_key(|a| a.0);
    res.iter().map(|a| hands_strs[a.0]).collect()
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Kind2 {
    Heart,
    Diamond,
    Clover,
    Spade,
    Joker,
}

impl From<&str> for Kind2 {
    fn from(value: &str) -> Self {
        match value {
            "H" => Kind2::Heart,
            "D" => Kind2::Diamond,
            "C" => Kind2::Clover,
            "S" => Kind2::Spade,
            a => panic!("Unrecognizable kind: {}", a),
        }
    }
}

impl Kind2 {
    pub fn eq_suit(&self, rhs: &Self) -> bool {
        matches!(self, Self::Heart | Self::Diamond);
        match self {
            Self::Heart | Self::Diamond => matches!(rhs, Self::Heart | Self::Diamond),
            Self::Clover | Self::Spade => matches!(rhs, Self::Clover | Self::Spade),
            Self::Joker => rhs == &Self::Joker,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Number(u8);

impl Number {
    fn succ(&self) -> Number {
        Self((self.0 + 1) % 14)
    }
}

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
        let sn = if self.0 == 12 { 14 } else { self.0 };
        let on = if other.0 == 12 { 14 } else { other.0 };
        sn.partial_cmp(&on)
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let num = (self.0 + rhs.0) % 14;
        Self(num)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Card {
    kind: Kind2,
    num: u8,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        assert!(2 <= value.len(), "Too short");
        let (num, kind) = value.split_at(value.len() - 1);
        const NUMBERS: [&str; 13] = [
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ];
        let num = NUMBERS
            .iter()
            .position(|&a| a == num)
            .unwrap_or_else(|| panic!("Unrecognizable number: {num}")) as u8
            + 1;
        let card = kind.into();
        Self { kind: card, num }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.kind.partial_cmp(&other.kind) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        let sn = if self.num == 12 { 100 } else { self.num };
        let on = if other.num == 12 { 100 } else { other.num };
        sn.partial_cmp(&on)
    }
}

#[derive(Debug, PartialEq)]
struct Hand(Vec<Card>);
// struct HandRes {
//     Hands
// }

impl Hand {
    fn hand(&self) -> (Hands, i8) {
        struct Res {
            flush: Option<u8>,
            straight: Option<u8>,
            comb: i32,
        }
        let res = Res {
            flush: self.is_flush(),
            straight: self.is_straight(),
            comb: self.count_same_number_combination(),
        };

        let _r2 = (
            self.is_flush(),
            self.is_straight(),
            self.count_same_number_combination(),
        );

        match res {
            Res {
                flush: Some(_),
                straight: Some(num),
                comb: _,
            } => (Hands::StraightFlush, num as i8),

            Res {
                flush: None,
                straight: Some(num),
                comb: _,
            } => (Hands::Straight, num as i8),

            Res {
                flush: Some(num),
                straight: None,
                comb: _,
            } => (Hands::Flush, num as i8),

            Res {
                flush: _,
                straight: _,
                comb: 6,
            } => (Hands::FourOfAKind, 0),

            Res {
                flush: _,
                straight: _,
                comb: 4,
            } => (Hands::FullHouse, 0),

            Res {
                flush: _,
                straight: _,
                comb: 3,
            } => (Hands::ThreeOfAKind, 0),

            Res {
                flush: _,
                straight: _,
                comb: 2,
            } => (Hands::TwoPair, 0),

            Res {
                flush: _,
                straight: _,
                comb: 1,
            } => (Hands::OnePair, 0),
            _ => (Hands::HighCard, 0),
        }
    }

    fn _hand(&self) -> (Hands, i8) {
        // by kind
        if self.0.windows(2).all(|pair| pair[0].kind == pair[1].kind) {
            return (Hands::Flush, 0);
        }
        // by number
        let mut nums = HashMap::new();
        self.0
            .iter()
            .map(|a| a.num)
            .for_each(|num| *nums.entry(num).or_insert(0) += 1);

        let max = nums.iter().max_by_key(|a| a.1).unwrap();
        match max {
            (number, 4) => return (Hands::FiveOfAKind, *number as i8),
            (number, 3) if nums.values().any(|a| *a == 2) => {
                return (Hands::FullHouse, *number as i8)
            }
            (number, 3) => return (Hands::ThreeOfAKind, *number as i8),
            (number, 2) if nums.values().filter(|&a| *a == 2).count() == 2 => {
                return (Hands::TwoPair, *number as i8)
            }
            (number, 2) => return (Hands::OnePair, *number as i8),
            _ => {}
        }

        let mut tmp = self.0.clone();
        tmp.sort_by_key(|a| a.num);
        if tmp.windows(2).all(|pair| pair[0].num + 1 == pair[1].num) {
            // return ;
        }
        unimplemented!()
    }

    fn is_flush(&self) -> Option<u8> {
        let max = self.0.iter().map(|a| a.num).max().unwrap();
        self.0
            .windows(2)
            .all(|pair| pair[0].kind.eq_suit(&pair[0].kind))
            .then_some(max)
    }

    fn is_straight(&self) -> Option<u8> {
        let mut tmp = self.0.clone();
        tmp.sort_by_key(|a| a.num);
        tmp.windows(2)
            .map(|a| (a[0].num, a[1].num))
            .all(|(a, b)| a + 1 == b)
            .then_some(tmp.last().unwrap().num as u8)
    }

    fn is_straight_flush(&self) -> Option<u8> {
        self.is_flush().and_then(|_| self.is_straight())
    }

    /// 6 => Four of a kind
    /// 4 => Full House
    /// 3 => Three of a kind
    /// 2 => Two pair
    /// 1 => One pair
    fn count_same_number_combination(&self) -> i32 {
        self.0
            .iter()
            .combinations(2)
            .filter(|pair| pair[0].num == pair[1].num)
            .count() as i32
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let cards: _ = value.split(' ').map(|a| a.into()).collect();
        Self(cards)
    }
}

// impl PartialOrd for Hand {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.0.partial_cmp(&other.0)
//     }
// }

// fn pair

#[derive(Debug, PartialEq, PartialOrd)]
enum Hands {
    // opt
    FiveOfAKind = 10,
    StraightFlush = 9,
    FourOfAKind = 8,
    FullHouse = 7,
    Flush = 6,
    Straight = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let a = "4S 5H 5S 5D 5C";
        let hand = Hand::from(a);

        // assert_eq!(
        //     hand,
        //     Hand(vec![
        //         Card(Kind::Spade(4)),
        //         Card(Kind::Heart(5)),
        //         Card(Kind::Spade(5)),
        //         Card(Kind::Diamond(5)),
        //         Card(Kind::Clover(5)),
        //     ])
        // );
    }
}
