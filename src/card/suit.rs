#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Suit {
    Heart,
    Diamond,
    Clover,
    Spade,
    Joker,
}

impl From<&str> for Suit {
    fn from(value: &str) -> Self {
        match value {
            "H" => Suit::Heart,
            "D" => Suit::Diamond,
            "C" => Suit::Clover,
            "S" => Suit::Spade,
            a => panic!("Unrecognizable kind: {a}"),
        }
    }
}
