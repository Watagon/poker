use std::{cell::RefCell, collections::HashMap, num, ops::Add};

use itertools::Itertools;
// use itertools::Itertools;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands_strs: &[&'a str]) -> Vec<&'a str> {
    // unimplemented!("Out of {hands:?}, which hand wins?")
    let hands: Vec<Hand> = hands_strs.iter().map(|&a| a.into()).collect();
    let res = hands
        .iter()
        // .map(|a| a.hand())
        .collect::<Vec<_>>();
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
    println!("{:?}", res.clone().collect::<Vec<_>>());
    res.clone().for_each(|a| println!("///{a:?}"));
    let a = res.clone().take(2).collect::<Vec<_>>();
    println!("|||{:?}", (*a[0].1).eq(*a[1].1));
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
    // eq-suit means two have same mark. (not color)
    // pub fn eq_suit(&self, rhs: &Self) -> bool {
    //     // matches!(self, Self::Heart | Self::Diamond);
    //     match self {
    //         Self::Heart | Self::Diamond => matches!(rhs, Self::Heart | Self::Diamond),
    //         Self::Clover | Self::Spade => matches!(rhs, Self::Clover | Self::Spade),
    //         Self::Joker => rhs == &Self::Joker,
    //     }
    // }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Number(u8);

impl Number {
    fn succ(&self) -> Number {
        Self((self.0 % 13) + 1)
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

#[derive(Debug, Clone)]
struct Card {
    kind: Kind2,
    num: Number,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        assert!(2 <= value.len(), "Too short");
        let (num, kind) = value.split_at(value.len() - 1);
        let card = kind.into();
        Self {
            kind: card,
            num: num.into(),
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // match self.kind.partial_cmp(&other.kind) {
        //     Some(core::cmp::Ordering::Equal) => {}
        //     ord => return ord,
        // }
        self.num.partial_cmp(&other.num)
    }
}

#[derive(Debug, PartialEq)]
struct Hand(Vec<Card>, RefCell<Option<(Hands, Vec<Number>)>>);
// struct HandRes {
//     Hands
// }

impl Hand {
    fn hand(&self) -> (Hands, Vec<Number>) {
        if let Some(hand) = self.1.borrow().clone() {
            return hand;
        }
        struct Res {
            flush: Option<Vec<Number>>,
            straight: Option<Vec<Number>>,
            comb: (i32, Vec<Number>),
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
        let def = vec![Number(2)];

        let ret = match res {
            Res {
                flush: Some(_),
                straight: Some(num),
                comb: _,
            } => (Hands::StraightFlush, num),

            Res {
                flush: None,
                straight: Some(num),
                comb: _,
            } => (Hands::Straight, num),

            Res {
                flush: Some(num),
                straight: None,
                comb: _,
            } => (Hands::Flush, num),

            Res {
                flush: _,
                straight: _,
                comb: (6, nums),
            } => (Hands::FourOfAKind, nums),

            Res {
                flush: _,
                straight: _,
                comb: (4, nums),
            } => (Hands::FullHouse, nums),

            Res {
                flush: _,
                straight: _,
                comb: (3, nums),
            } => (Hands::ThreeOfAKind, nums),

            Res {
                flush: _,
                straight: _,
                comb: (2, nums),
            } => (Hands::TwoPair, nums),

            Res {
                flush: _,
                straight: _,
                comb: (1, nums),
            } => (Hands::OnePair, nums),
            _ => (Hands::HighCard, def),
        };
        *self.1.borrow_mut() = Some(ret.clone());
        ret
    }

    fn is_flush(&self) -> Option<Vec<Number>> {
        let max = self.0.iter().map(|a| a.num).max().unwrap();
        self.0
            .windows(2)
            // .all(|pair| pair[0].kind.eq_suit(&pair[1].kind))
            .all(|pair| pair[0].kind == pair[1].kind)
            .then(|| {
                let mut nums = self.0.iter().map(|a| a.num).collect::<Vec<_>>();
                nums.sort();
                nums.reverse();
                nums
            })
    }

    fn is_straight(&self) -> Option<Vec<Number>> {
        let mut tmp = self.0.iter().map(|a| a.num).collect::<Vec<_>>();
        tmp.sort();

        let _is_straight = |tmp: &mut [Number]| {
            tmp.windows(2)
                .map(|a| (a[0], a[1]))
                .all(|(a, b)| a + 1 == b)
                // .then_some(tmp.last().unwrap().num)
                .then(|| {
                    tmp.reverse();
                    Vec::from(tmp)
                })
        };
        _is_straight(&mut tmp)
            // aces can start a straight (A 2 3 4 5)
            .or_else(|| {
                let ace = tmp.pop().unwrap();
                if ace == Number(1) {
                    tmp.insert(0, ace);
                    _is_straight(&mut tmp)
                } else {
                    None
                }
            })
    }

    fn is_straight_flush(&self) -> Option<Vec<Number>> {
        self.is_flush().and_then(|_| self.is_straight())
    }

    /// 6 => Four of a kind
    /// 4 => Full House
    /// 3 => Three of a kind
    /// 2 => Two pair
    /// 1 => One pair
    /// compare with `Vec<Number>`
    /// first element is more important to rank
    fn count_same_number_combination(&self) -> (i32, Vec<Number>) {
        let mut duped = self
            .0
            .iter()
            .map(|a| a.num)
            .combinations(2)
            .filter_map(|pair| (pair[0] == pair[1]).then_some(pair[0]))
            .collect::<Vec<_>>();
        let count = duped.len();
        duped.sort();
        // Ascend
        let counted = {
            let cn = duped.iter().map(|a| a.clone()).counts();
            let mut cnv = cn.into_iter().collect::<Vec<_>>();
            // both hands have a full house, tie goes to highest-ranked triplet
            cnv.sort_by_key(|a| a.0); // secondary key
            cnv.sort_by_key(|a| a.1); // primary key
            // cnv.reverse();
            println!("cnv: {cnv:?}");
            cnv
        };
        let mut nums = counted.into_iter().map(|a|a.0).rev().collect::<Vec<_>>();
        println!("nums: {nums:?}");
        // not duped numbers
        let mut remained = self.0.iter().map(|a|a.num).filter(|a|!nums.contains(a)).collect::<Vec<_>>();
        remained.sort();
        nums.extend(remained.into_iter().rev());

        (count as i32, nums)
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let cards: _ = value.split(' ').map(|a| a.into()).collect();
        Self(cards, RefCell::new(None))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let sh = self.hand();
        let oh = other.hand();
        if sh.0 == oh.0 {
            let sn = &self.0;
            let on = &other.0;
            match sh.0 {
                Hands::FiveOfAKind => Some(std::cmp::Ordering::Equal),
                Hands::StraightFlush => sh.1.partial_cmp(&oh.1),
                Hands::FourOfAKind => sh.1.partial_cmp(&oh.1),
                Hands::FullHouse => sh.1.partial_cmp(&oh.1),
                Hands::Flush => sh.1.partial_cmp(&oh.1),
                Hands::Straight => sh.1.partial_cmp(&oh.1),
                Hands::ThreeOfAKind => sh.1.partial_cmp(&oh.1),
                Hands::TwoPair => sh.1.partial_cmp(&oh.1),
                Hands::OnePair => sh.1.partial_cmp(&oh.1),
                Hands::HighCard => {
                    let mut sn = self.0.clone();
                    let mut on = other.0.clone();
                    sn.sort_by_key(|a| a.num);
                    on.sort_by_key(|a| a.num);
                    sn.reverse();
                    on.reverse();
                    // println!("L==={sn:#?}");
                    // println!("R==={on:#?}");
                    // println!("=========={:?}", sn.partial_cmp(&on));
                    sn.partial_cmp(&on)
                }
            }
        } else {
            sh.0.partial_cmp(&oh.0)
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

    // #[test]
    // fn kind2_eq_suit() {
    //     use Kind2::*;
    //     assert!(Heart.eq_suit(&Diamond));
    //     assert!(Diamond.eq_suit(&Heart));
    //     assert!(!Heart.eq_suit(&Spade));
    //     assert!(!Heart.eq_suit(&Clover));
    // }

    #[test]
    fn test_aces_a_straight_high() {
        // aces can end a straight (10 J Q K A)
        assert_eq!(
            winning_hands(&["AS 2H 3C 4D 5H", "10D JH QS KD AC"]),
            &["10D JH QS KD AC"]
        )
    }

    #[test]
    fn card_eq() {
        let a = Card {
            kind: Kind2::Clover,
            num: Number(1),
        };
        let b = Card {
            kind: Kind2::Diamond,
            num: Number(1),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_full_house_ranks_1() {
        // both hands have a full house, tie goes to highest-ranked triplet
        assert_eq!(
            winning_hands(&["4H 4S 4D 9S 9D", "5H 5S 5D 8S 8D"]),
            &["5H 5S 5D 8S 8D"]
        )
    }

    #[test]
    fn number_sort_ord(){
        let a = Number(1);
        let b = Number(2);
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
    }
}
