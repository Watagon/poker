#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum Suit {
    Heart,
    Diamond,
    Clover,
    Spade,
    _Joker,
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
