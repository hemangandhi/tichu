extern crate rand;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::tichu::hand;

pub type PlayerCards = [hand::Card; 14];

pub static deck: [hand::Card; 14 * 4] = {
    let mut cards: [hand::Card; 14 * 4];
    let mut idx = 0;
    for suit in hand::normal_suits.iter() {
        for value in hand::Value::Dog {
            if value > hand::Value::Ace {
                break;
            }

            cards[idx] = hand::Card {
                suit: *suit,
                value: value,
            };
            idx += 1;
        }
    }
    cards
};

pub struct Game {
    pub players: [PlayerCards; 4],
    pub slash_score: i32, // slash and cross are arbitrary, just
    pub cross_score: i32, // to distinguish the teams (players know
                          // about their partner)
}

impl Game {
    pub fn New() -> Self {
        let mut shuffled: [hand::Card; 14 * 4];
        shuffled.copy_from_slice(&deck);
        shuffled.shuffle(&mut thread_rng());
    }
}
