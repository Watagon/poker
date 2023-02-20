mod card;
mod hand;

pub use card::{Card, Number};
pub use hand::Hand;

use itertools::Itertools;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands_strs: &[&'a str]) -> Vec<&'a str> {
    // unimplemented!("Out of {hands:?}, which hand wins?")
    hands_strs
        .iter()
        .map(|&a| Hand::from(a))
        .enumerate()
        .sorted_by(|a, b| {
            a.1.partial_cmp(&b.1)
                .unwrap_or_else(|| panic!("Err: partial_cmp: a: {a:?}, b: {b:?}"))
        })
        .rev()
        .fold(Vec::<(usize, Hand)>::new(), |mut acc, x| {
            if acc.is_empty() || acc.first().unwrap().1.eq(&x.1) {
                acc.push(x);
            }
            acc
        })
        .into_iter()
        .sorted_by_key(|a| a.0)
        .map(|a| hands_strs[a.0])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{card::Suit, hand::Hand};

    #[test]
    fn from() {
        let a = "4S 5H 5S 5D 5C";
        let _hand = Hand::from(a);

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
            kind: Suit::Clover,
            num: Number(1),
        };
        let b = Card {
            kind: Suit::Diamond,
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
    fn number_sort_ord() {
        let a = Number(1);
        let b = Number(2);
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
    }
}
