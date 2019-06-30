use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
pub enum Suit{
    House,
    Star,
    Sword,
    Jade,
    Special
}

#[derive(PartialEq, Eq, Debug)]
pub enum Value{
    Dog, //Mahjong = Numeric(1)
    Numeric(u32),
    Jack,
    Queen,
    King,
    Ace,
    Pheonix,
    Dragon
}

#[derive(PartialEq, Eq, Debug)]
pub struct Card{
    pub suit: Suit,
    pub value: Value
}

impl Ord for Card{
    //self is <ret val> than other
    //hecking gross, but done
    fn cmp(&self, other: &Self) -> Ordering {
        if other.value == self.value {
            return Ordering::Equal;
        }

        match self.value {
            Value::Dragon => Ordering::Greater,
            Value::Pheonix => {
                if other.value == Value::Dragon {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            },
            Value::Ace => {
                if other.value == Value::Dragon
                    || other.value == Value::Pheonix {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            },
            Value::King => {
                if other.value == Value::Ace || other.value == Value::Dragon
                    || other.value == Value::Pheonix {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            },
            Value::Queen => {
                if other.value == Value::King || other.value == Value::Ace
                    || other.value == Value::Dragon
                    || other.value == Value::Pheonix {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            },
            Value::Jack => {
                if other.value == Value::Queen || other.value == Value::King
                    || other.value == Value::Ace
                    || other.value == Value::Dragon
                    || other.value == Value::Pheonix {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            },
            Value::Numeric(self_n) => {
                if let Value::Numeric(other_n) = other.value {
                    self_n.cmp(&other_n)
                } else if other.value == Value::Dog {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            },
            Value::Dog => Ordering::Greater
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum HandType{
    Single(Card),
    Pair(Card),
    Triple(Card),
    FourOfAKind(Card),
    FullHouse(Card, Card), //first over second
    ConsecutivePairs(Card, u32), //bottom and length
    Straight(Card, u32), //bottom and length
    StraightFlush(Card, u32)
}

#[derive(PartialEq, Eq, Debug)]
pub struct Hand{
    rank: HandType,
    cards: Vec<Card>
}

impl Hand {
    pub fn is_bomb(&self) -> bool {
        match self.rank {
            HandType::StraightFlush(_, _) | HandType::FourOfAKind(_) => true,
            _ => false
        }
    }

    pub fn probability_of_being_beaten(&self, unseen_cards: Vec<Card>) -> f64 {
        let winners: u32 = match &self.rank {
            HandType::Single(card) => {
                unseen_cards.iter().filter(|&unseen| card < unseen).count();
            },
            HandType::Pair(card) => {
                unseen_cards.iter().filter(|&unseen| {
                    card < unseen && unseen_cards.iter().any(|&pairer| {
                        pairer.value == unseen.value && pairer != unseen
                    })
                }).count() / 2
            },
            HandType::Triple(card) => {
                unseen_cards.iter().filter(|&unseen| {
                    card < unseen &&
                    unseen_cards.iter().filter(|&pairer| {
                        pairer.value == unseen.value && pairer != unseen
                    }).count() == 2
                }).count() / 3
            },
            HandType::FourOfAKind(card) => {
                unseen_cards.iter().filter(|&unseen| {
                    card < unseen &&
                    unseen_cards.iter().filter(|&pairer| {
                        pairer.value == unseen.value && pairer != unseen
                    }).count() == 3
                }).count() / 4
            }
        }
    }
}

//This comparision is for trick-taking/playing legalities...
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match &self.rank  {
            HandType::Single(self_card) => {
                if let HandType::Single(other_card) = &other.rank {
                    Option::Some(self_card.cmp(&other_card))
                } else if other.is_bomb() {
                    Option::Some(Ordering::Less)
                } else {
                    Option::None
                }
            },
            HandType::Pair(self_card) => {
                if let HandType::Pair(other_card) = &other.rank {
                    Option::Some(self_card.cmp(&other_card))
                } else if other.is_bomb() {
                    Option::Some(Ordering::Less)
                } else {
                    Option::None
                }
            },
            HandType::Triple(self_card) => {
                if let HandType::Triple(other_card) = &other.rank {
                    Option::Some(self_card.cmp(&other_card))
                } else if other.is_bomb() {
                    Option::Some(Ordering::Less)
                } else {
                    Option::None
                }
            },
            HandType::FourOfAKind(self_card) => {
                if let HandType::FourOfAKind(other_card) = &other.rank {
                    Option::Some(self_card.cmp(&other_card))
                } else if let HandType::StraightFlush(_, _) = &other.rank {
                    Option::Some(Ordering::Less)
                } else {
                    Option::None
                }
            },
            HandType::FullHouse(self_card, _) => {
                if let HandType::FullHouse(other_card, _) = &other.rank {
                    Option::Some(self_card.cmp(&other_card))
                } else if other.is_bomb() {
                    Option::Some(Ordering::Less)
                } else {
                    Option::None
                }
            },
            HandType::ConsecutivePairs(self_card, len) => {
                if let HandType::ConsecutivePairs(other_card, len2) = &other.rank {
                    if len == len2 {
                        Option::Some(self_card.cmp(&other_card))
                    } else {
                        Option::None
                    }
                } else if other.is_bomb() {
                    Option::Some(Ordering::Less)
                } else {
                    Option::None
                }
            },
            HandType::Straight(self_card, len) => {
                if let HandType::Straight(other_card, len2) = &other.rank {
                    if len == len2 {
                        Option::Some(self_card.cmp(&other_card))
                    } else {
                        Option::None
                    }
                } else if other.is_bomb() {
                    Option::Some(Ordering::Less)
                } else {
                    Option::None
                }
            },
            HandType::StraightFlush(self_card, len) => {
                if let HandType::StraightFlush(other_card, len2) = &other.rank {
                    if len == len2 {
                        Option::Some(self_card.cmp(&other_card))
                    } else {
                        Option::None
                    }
                } else {
                    Option::None
                }
            }
        }
    }
}
