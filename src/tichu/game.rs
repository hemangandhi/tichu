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

#[derive(Debug)]
pub struct Game {
    pub players: [PlayerCards; 4],
    pub slash_score: i32, // slash and cross are arbitrary, just
    pub cross_score: i32, // to distinguish the teams (players know
    // about their partner)
    turn_index: u8,
    tichu_calls: [Option<bool>; 4],
}

pub trait Player {
    // maintaining the unseen cards is the player's perogative
    // the bool should be true if the player seeks to call tichu.
    fn play(&mut self, own_hand: PlayerCards) -> (hand::Hand, bool);
    // the bool returned is intent to bomb
    fn record_other_play(&mut self, play: &hand::Hand, is_partner: bool, calls_tichu: bool)
        -> bool;
}

impl Game {
    pub fn new() -> Self {
        let mut shuffled: [hand::Card; 14 * 4] = [hand::Card {
            value: hand::Value::Dog,
            suit: hand::Suit::Special,
        }; 14 * 4];
        shuffled.copy_from_slice(&deck);
        shuffled.shuffle(&mut thread_rng());

        unsafe {
            let mut turn_index = 0;
            let mut hands: [PlayerCards; 4] = std::mem::uninitialized();
            let mut i = 0;
            for hand in shuffled.chunks(14) {
                let mut j = 0;
                for card in hand {
                    hands[i as usize][j] = Option::Some(*card);
                    if card.value == hand::Value::Numeric(1) {
                        // they're bytes ffs
                        turn_index = i;
                    }
                    j += 1;
                }
                i += 1;
            }
            Game {
                players: hands,
                slash_score: 0,
                cross_score: 0,
                turn_index: turn_index,
                // why the fresh fuck can't I copy bools?!
                tichu_calls: [Option::None, Option::None, Option::None, Option::None],
            }
        }
    }

    fn play_move_helper<T: Player>(
        &mut self,
        curr_player: &mut T,
        other_players: [&mut Option<&mut T>; 4],
    ) {
        let curr_hand = &self.players[self.turn_index as usize];
        let (hand, calls_tichu) = curr_player.play(*curr_hand).clone();
        self.tichu_calls[self.turn_index as usize] =
            match self.tichu_calls[self.turn_index as usize] {
                Option::None => match hand.rank {
                    hand::HandType::Pass => Option::None,
                    _ => Option::Some(calls_tichu),
                },
                x => x,
            };
        let mut broke = false;
        for i in 0..4 {
            if i == self.turn_index {
                continue;
            }
            if let Option::Some(player) = other_players[i as usize] {
                broke = player.record_other_play(
                    &hand,
                    (i as i8 - self.turn_index as i8).abs() == 2,
                    calls_tichu,
                );
                if broke {
                    self.turn_index = i as u8;
                    break;
                }
            }
        }

        for idx in 0..14 {
            self.players[self.turn_index as usize][idx] =
                if let Option::Some(card) = self.players[self.turn_index as usize][idx] {
                    if hand.cards.iter().any(|crd| *crd == card) {
                        Option::None
                    } else {
                        Option::Some(card)
                    }
                } else {
                    Option::None
                }
        }

        if !broke {
            self.turn_index = (self.turn_index + 1) % 4;
        }
    }

    pub fn play_move<T: Player>(
        &mut self,
        player1: &mut T,
        player2: &mut T,
        player3: &mut T,
        player4: &mut T,
    ) {
        match self.turn_index {
            0 => self.play_move_helper(
                player1,
                [
                    &mut Option::None,
                    &mut Option::Some(player2),
                    &mut Option::Some(player3),
                    &mut Option::Some(player4),
                ],
            ),
            1 => self.play_move_helper(
                player2,
                [
                    &mut Option::Some(player1),
                    &mut Option::None,
                    &mut Option::Some(player3),
                    &mut Option::Some(player4),
                ],
            ),
            2 => self.play_move_helper(
                player3,
                [
                    &mut Option::Some(player1),
                    &mut Option::Some(player2),
                    &mut Option::None,
                    &mut Option::Some(player4),
                ],
            ),
            3 => self.play_move_helper(
                player4,
                [
                    &mut Option::Some(player1),
                    &mut Option::Some(player2),
                    &mut Option::Some(player3),
                    &mut Option::None,
                ],
            ),
            _ => {}
        }
    }
}