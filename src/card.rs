mod number;
mod suit;

pub use number::Number;
pub use suit::Suit;

#[derive(Debug, Clone)]
pub struct Card {
    pub kind: Suit,
    pub num: Number,
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
        self.num.partial_cmp(&other.num)
    }
}
