#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum Hands {
    // opt
    _FiveOfAKind = 10,
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
