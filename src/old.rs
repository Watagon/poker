/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands_strs: &[&'a str]) -> Vec<&'a str> {
    // unimplemented!("Out of {hands:?}, which hand wins?")
    let hands: Vec<Hand> = hands_strs.iter().map(|&a|a.into()).collect();
    unimplemented!()
}

#[derive(Debug,  PartialEq, PartialOrd)]
pub enum Kind {
    Heart(u8),
    Diamond(u8),
    Clover(u8),
    Spade(u8),
    Joker,
}


#[derive(Debug)]
pub enum Kind2 {
    Heart,
    Diamond,
    Clover,
    Spade,
    Joker,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Card(Kind);

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        assert!(2 <= value.len(), "Too short");
        let (num, kind) = value.split_at(value.len() - 1);
        const NUMBERS: [&str; 13] = [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ];
        let num = NUMBERS
            .iter()
            .position(|&a| a == num)
            .expect("Unrecognizable number") as u8
            + 1;
        let card = match kind {
            "H" => Kind::Heart(num),
            "D" => Kind::Diamond(num),
            "C" => Kind::Clover(num),
            "S" => Kind::Spade(num),
            a => panic!("Unrecognizable kind: {}", a),
        };
        Self(card)
    }
}

#[derive(Debug, PartialEq)]
struct Hand(Vec<Card>);

impl Hand {
    fn hand(&self) -> Hands {
        // by kind
        self.0.windows(2).all(|[a, b]| {
            
        })
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let cards: _ = value.split(' ').map(|a| a.into()).collect();
        Self(cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unimplemented!()
    }
}

// fn pair

#[derive(Debug, PartialEq, PartialOrd)]
enum Hands {
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

        assert_eq!(
            hand,
            Hand(vec![
                Card(Kind::Spade(4)),
                Card(Kind::Heart(5)),
                Card(Kind::Spade(5)),
                Card(Kind::Diamond(5)),
                Card(Kind::Clover(5)),
            ])
        );
    }
}
