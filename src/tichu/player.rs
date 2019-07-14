extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::tichu::hand;

pub type PlayerCards = [Option<hand::Card>; 14];

pub static deck: [hand::Card; 14 * 4] = {
    let cards: &mut [hand::Card; 14 * 4] = &mut [hand::Card {value: hand::Value::Dog, suit: hand::Suit::Special}; 14 * 4];
    let mut idx = 0;
    for suit in &hand::normal_suits {
        for value in &hand::normal_values {
            cards[idx] = hand::Card {
                suit: *suit,
                value: *value,
            };
            idx += 1;
        }
    }

    cards[idx] = hand::Card {
        suit: hand::Suit::Special,
        value: hand::Value::Dog
    };
    cards[idx + 1] = hand::Card {
        suit: hand::Suit::Special,
        value: hand::Value::Numeric(1)
    };
    cards[idx + 2] = hand::Card {
        suit: hand::Suit::Special,
        value: hand::Value::Pheonix
    };
    cards[idx + 3] = hand::Card {
        suit: hand::Suit::Special,
        value: hand::Value::Dragon
    };
    *cards
};

pub struct Game<'a> {
    pub players: [&'a mut PlayerCards; 4],
    pub slash_score: i32, // slash and cross are arbitrary, just
    pub cross_score: i32, // to distinguish the teams (players know
                          // about their partner)
}

impl<'a> Game<'a> {
    pub fn New() -> Self {
        let mut shuffled: [hand::Card; 14 * 4];
        shuffled.copy_from_slice(&deck);
        shuffled.shuffle(&mut thread_rng());

        let hands: [&mut PlayerCards; 4];
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
