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

impl Value {
    pub fn next_value(&self) -> Self {
        //technically, this method should always also return
        //Value::Pheonix but y'know, it don't
        match &self {
            Value::Dog => Value::Numeric(1),
            Value::Numeric(n) => if n.to_owned() <= 9 { Value::Numeric(n + 1) }
                                 else { Value::Jack },
            Value::Jack => Value::Queen,
            Value::Queen => Value::King,
            Value::King => Value::Ace,
            Value::Ace => Value::Dragon,
            //Just to satisfy the compiler
            Value::Pheonix => Value::Pheonix,
            Value::Dragon => Value::Dragon,
        }
    }

    pub fn values_that_beat(&self) -> impl Iterator<Item=Self> {
        self.into_iter()
    }
}

impl Iterator for Value {
    type Item=Self;

    fn next(&mut self) -> Option<Self> {
        if self.next_value() == *self {
            Option::None
        } else {
            Option::Some(self.next_value())
        }
    }

}

#[derive(Eq, Debug)]
pub struct Card{
    pub suit: Suit,
    pub value: Value
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suit == other.suit
    }
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

impl HandType {
    pub fn num_cards(&self) -> u32 {
        match &self {
            HandType::Single(_) => 1,
            HandType::Pair(_) => 2,
            HandType::Triple(_) => 3,
            HandType::FourOfAKind(_) => 4,
            HandType::FullHouse(_, _) => 5,
            HandType::ConsecutivePairs(_, len) => 2 * len.to_owned(),
            HandType::Straight(_, len) => len.to_owned(),
            HandType::StraightFlush(_, len) => len.to_owned()
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Hand{
    pub rank: HandType,
    pub cards: Vec<Card>
}

cached!{
    NCR;
    fn ncr(n: u32, r:u32) -> u32 = {
        if n < r { 0 }
        else if n == r || r == 0 { 1 }
        else { ncr(n - 1, r - 1) + ncr(n - 1, r) }
    }
}

impl Hand {
    pub fn is_bomb(&self) -> bool {
        match self.rank {
            HandType::StraightFlush(_, _) | HandType::FourOfAKind(_) => true,
            _ => false
        }
    }

    fn groups_of_n_that_beat(n: u32, card: Card, use_pheonix: bool) -> usize {
        
    }

    //hypocritical in that bombs themselves should be passed in here
    fn num_non_bomb_plays_that_beat(&self, unseen_cards: &Vec<Card>) -> usize {
        match &self.rank {
            HandType::Single(card) => {
                unseen_cards.iter().filter(|&unseen| card < unseen).count()
            },
            HandType::Pair(card) => {
                unseen_cards.iter().map(|&unseen| {
                    if card < &unseen {
                        unseen_cards.iter().filter(|&pairer| {
                            (pairer.value == unseen.value && pairer != &unseen)
                                || pairer.value == Value::Pheonix     // the division by 2 later makes
                                || (unseen.value == Value::Pheonix && // it better for us to just be sure
                                    pairer > card)                    // we're double-counting
                        }).count()
                    } else { 0 }
                }).sum::<usize>() / 2
            },
            HandType::Triple(card) => {
                unseen_cards.iter().filter(|&unseen| {
                    card < unseen && unseen.value != Value::Pheonix &&
                    unseen_cards.iter().filter(|&pairer| {
                        pairer.value == unseen.value && pairer != unseen
                    }).count() >= 2
                }).count() / 3 +
                //here, this is the easier way to account for the pheonix
                if unseen_cards.iter().any(|&u| u.value == Value::Pheonix) {
                    unseen_cards.iter().map(|&unseen| {
                        if &unseen > card {
                            unseen_cards.iter()
                            .filter(|&p| p != &unseen && p.value == unseen.value)
                            .count()
                        } else { 0 }
                    }).sum::<usize>() / 2 //we still double-count the pheonix-free pairs
                } else { 0 }
            },
            HandType::FourOfAKind(card) => {
                unseen_cards.iter().filter(|&unseen| {
                    card < unseen &&
                    unseen_cards.iter().filter(|&pairer| {
                        pairer.value == unseen.value && pairer != unseen
                    }).count() == 3
                }).count() / 4
            },
            HandType::FullHouse(card, _) => {
                unseen_cards.iter().map(|&unseen: &Card| -> usize {
                    if card < &unseen &&
                        unseen_cards.iter().filter(|&pairer| {
                            pairer.value == unseen.value && pairer != &unseen
                        }).count() >= 2 {
                        unseen_cards.iter().filter(|&carried|{
                            carried.value != unseen.value &&
                            unseen_cards.iter().any(|&carried_pair| {
                                carried.value == carried_pair.value
                            })
                        }).count() / 2
                    } else { 0 }
                }).sum::<usize>() / 3
            },
            //TODO: the arms below here are just to compile and are
            //otherwise absurd
            HandType::ConsecutivePairs(card, length) => {
                unseen_cards.iter().map(|&unseen: &Card| -> usize {
                    if card < &unseen &&
                        unseen_cards.iter().filter(|&pairer| {
                            pairer.value == unseen.value && pairer != &unseen
                        }).count() >= 2 {
                        unseen_cards.iter().filter(|&carried|{
                            carried.value != unseen.value &&
                            unseen_cards.iter().any(|&carried_pair| {
                                carried.value == carried_pair.value
                            })
                        }).count() / 2
                    } else { 0 }
                }).sum::<usize>() / 3
            },
            HandType::Straight(card, length) => {
                unseen_cards.iter().map(|&unseen: &Card| -> usize {
                    if card < &unseen &&
                        unseen_cards.iter().filter(|&pairer| {
                            pairer.value == unseen.value && pairer != &unseen
                        }).count() >= 2 {
                        unseen_cards.iter().filter(|&carried|{
                            carried.value != unseen.value &&
                            unseen_cards.iter().any(|&carried_pair| {
                                carried.value == carried_pair.value
                            })
                        }).count() / 2
                    } else { 0 }
                }).sum::<usize>() / 3
            },
            HandType::StraightFlush(card, length) => {
                unseen_cards.iter().map(|&unseen: &Card| -> usize {
                    if card < &unseen &&
                        unseen_cards.iter().filter(|&pairer| {
                            pairer.value == unseen.value && pairer != &unseen
                        }).count() >= 2 {
                        unseen_cards.iter().filter(|&carried|{
                            carried.value != unseen.value &&
                            unseen_cards.iter().any(|&carried_pair| {
                                carried.value == carried_pair.value
                            })
                        }).count() / 2
                    } else { 0 }
                }).sum::<usize>() / 3
            }
        }
    }

    pub fn probability_of_being_beaten(&self, unseen_cards: &Vec<Card>,
                                       opp_hand_size: u32) -> f64 {
        //number of hands that beat self * (number of cards left choose # other cards in hand)
        //----------------------------------------------------------------------------------
        //            (number of cards left choose the # cards in hand)
        let opp_hand_freedom: u32 = opp_hand_size - self.rank.num_cards();
        if opp_hand_freedom < 0 { return 0f64; }

        //TODO: fuck all this casting
        //But also accounting for bombs worsens this, so there might have to be
        //multiple functions
        let winners: u32 = self.num_non_bomb_plays_that_beat(unseen_cards) as u32;
        let numerator = (winners * ncr(unseen_cards.len() as u32, opp_hand_freedom)) as f64;
        let denominator = ncr(unseen_cards.len() as u32, opp_hand_size) as f64;
        return numerator / denominator;
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
