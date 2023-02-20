mod hands;

use hands::Hands;

use crate::{Card, Number};
use itertools::Itertools;
use std::cell::RefCell;

#[derive(Debug, PartialEq)]
pub(crate) struct Hand(Vec<Card>, RefCell<Option<(Hands, Vec<Number>)>>);

impl Hand {
    fn hand(&self) -> (Hands, Vec<Number>) {
        if let Some(hand) = self.1.borrow().clone() {
            return hand;
        }

        let res = (
            self.is_flush(),
            self.is_straight(),
            self.count_same_number_combination(),
        );
        let def = vec![Number(2)];

        let ret = match res {
            (Some(_), Some(num), _) => (Hands::StraightFlush, num),
            (None, Some(num), _) => (Hands::Straight, num),
            (Some(num), None, _) => (Hands::Flush, num),
            (_, _, (6, nums)) => (Hands::FourOfAKind, nums),
            (_, _, (4, nums)) => (Hands::FullHouse, nums),
            (_, _, (3, nums)) => (Hands::ThreeOfAKind, nums),
            (_, _, (2, nums)) => (Hands::TwoPair, nums),
            (_, _, (1, nums)) => (Hands::OnePair, nums),
            _ => (Hands::HighCard, def),
        };
        *self.1.borrow_mut() = Some(ret.clone());
        ret
    }

    fn is_flush(&self) -> Option<Vec<Number>> {
        self.0
            .windows(2)
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

    /// 6 => Four of a kind
    /// 4 => Full House
    /// 3 => Three of a kind
    /// 2 => Two pair
    /// 1 => One pair
    /// compare with `Vec<Number>`
    /// first element is more important to rank
    fn count_same_number_combination(&self) -> (i32, Vec<Number>) {
        let duped = self
            .0
            .iter()
            .map(|a| a.num)
            .combinations(2)
            .filter_map(|pair| (pair[0] == pair[1]).then_some(pair[0]))
            .collect::<Vec<_>>();
        let count = duped.len();
        // Ascend
        let counted = {
            let cn = duped.into_iter().counts();
            let mut cnv = cn.into_iter().collect::<Vec<_>>();
            // both hands have a full house, tie goes to highest-ranked triplet
            cnv.sort_by_key(|a| a.0); // secondary key
            cnv.sort_by_key(|a| a.1); // primary key
            println!("cnv: {cnv:?}");
            cnv
        };
        let mut nums = counted.into_iter().map(|a| a.0).rev().collect::<Vec<_>>();
        // not duped numbers
        let mut remained = self
            .0
            .iter()
            .map(|a| a.num)
            .filter(|a| !nums.contains(a))
            .collect::<Vec<_>>();
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
            use Hands::*;
            match sh.0 {
                _FiveOfAKind => Some(std::cmp::Ordering::Equal),
                StraightFlush | FourOfAKind | FullHouse | Flush | Straight | ThreeOfAKind
                | TwoPair | OnePair => sh.1.partial_cmp(&oh.1),
                HighCard => {
                    let mut sn = self.0.clone();
                    let mut on = other.0.clone();
                    sn.sort_by_key(|a| a.num);
                    on.sort_by_key(|a| a.num);
                    sn.reverse();
                    on.reverse();
                    sn.partial_cmp(&on)
                }
            }
        } else {
            sh.0.partial_cmp(&oh.0)
        }
    }
}
