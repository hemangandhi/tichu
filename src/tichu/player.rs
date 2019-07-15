extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::tichu::hand;

pub type PlayerCards = [Option<hand::Card>; 14];

// Forgive me father, for I have sinned...
// TODO: use `const fn` to make this not trash
pub static deck: [hand::Card; 14 * 4] = [
    hand::Card {
        value: hand::Value::Numeric(2),
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::Numeric(2),
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::Numeric(2),
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::Numeric(2),
        suit: hand::Suit::Jade,
    },
    hand::Card {
        value: hand::Value::Numeric(3),
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::Numeric(3),
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::Numeric(3),
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::Numeric(3),
        suit: hand::Suit::Jade,
    },
    hand::Card {
        value: hand::Value::Numeric(4),
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::Numeric(4),
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::Numeric(4),
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::Numeric(4),
        suit: hand::Suit::Jade,
    },
    hand::Card {
        value: hand::Value::Numeric(5),
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::Numeric(5),
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::Numeric(5),
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::Numeric(5),
        suit: hand::Suit::Jade,
    },
    hand::Card {
        value: hand::Value::Numeric(6),
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::Numeric(6),
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::Numeric(6),
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::Numeric(6),
        suit: hand::Suit::Jade,
    },
    hand::Card {
        value: hand::Value::Numeric(7),
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::Numeric(7),
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::Numeric(7),
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::Numeric(7),
        suit: hand::Suit::Jade,
    },
    hand::Card {
        value: hand::Value::Numeric(8),
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::Numeric(8),
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::Numeric(8),
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::Numeric(8),
        suit: hand::Suit::Jade,
    },
    hand::Card {
        value: hand::Value::Numeric(9),
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::Numeric(9),
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::Numeric(9),
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::Numeric(9),
        suit: hand::Suit::Jade,
    },
    hand::Card {
        value: hand::Value::Numeric(10),
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::Numeric(10),
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::Numeric(10),
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::Numeric(10),
        suit: hand::Suit::Jade,
    },
    hand::Card {
        value: hand::Value::Jack,
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::Jack,
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::Jack,
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::Jack,
        suit: hand::Suit::Jade,
    },
    hand::Card {
        value: hand::Value::Queen,
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::Queen,
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::Queen,
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::Queen,
        suit: hand::Suit::Jade,
    },
    hand::Card {
        value: hand::Value::King,
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::King,
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::King,
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::King,
        suit: hand::Suit::Jade,
    },
    hand::Card {
        value: hand::Value::Ace,
        suit: hand::Suit::House,
    },
    hand::Card {
        value: hand::Value::Ace,
        suit: hand::Suit::Star,
    },
    hand::Card {
        value: hand::Value::Ace,
        suit: hand::Suit::Sword,
    },
    hand::Card {
        value: hand::Value::Ace,
        suit: hand::Suit::Jade,
    },
    // Tishu-specific
    hand::Card {
        value: hand::Value::Numeric(1),
        suit: hand::Suit::Special,
    },
    hand::Card {
        value: hand::Value::Dog,
        suit: hand::Suit::Special,
    },
    hand::Card {
        value: hand::Value::Pheonix,
        suit: hand::Suit::Special,
    },
    hand::Card {
        value: hand::Value::Dragon,
        suit: hand::Suit::Special,
    },
];

pub struct Game<'a> {
    pub players: [&'a mut PlayerCards; 4],
    pub slash_score: i32, // slash and cross are arbitrary, just
    pub cross_score: i32, // to distinguish the teams (players know
                          // about their partner)
}

impl<'a> Game<'a> {
    pub fn New() -> Self {
        let mut shuffled: [hand::Card; 14 * 4] = [hand::Card {
            value: hand::Value::Dog,
            suit: hand::Suit::Special,
        }; 14 * 4];
        shuffled.copy_from_slice(&deck);
        shuffled.shuffle(&mut thread_rng());

        unsafe {
            let hands: [&'a mut PlayerCards; 4] = std::mem::uninitialized();
            let mut i = 0;
            for hand in shuffled.chunks(14) {
                let mut j = 0;
                for card in hand {
                    hands[i][j] = Option::Some(*card);
                    j += 1;
                }
                i += 1
            }
            Game {
                players: hands,
                slash_score: 0,
                cross_score: 0,
            }
        }

    }
}
