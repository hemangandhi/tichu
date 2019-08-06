extern crate rand;

use std::cmp::Ordering;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TichuCall {
    NotYetPlayed,
    NoCall,
    ///Arbitrary conflict resolution: if you and a partner
    ///call, the more confident one is picked and the other
    ///notified. The confidences are set to 0 when passed.
    Tichu(f64),
    GrandTichu(f64),
}

impl Eq for TichuCall {}

impl PartialOrd for TichuCall {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::Some(self.cmp(other))
    }
}

impl Ord for TichuCall {
    fn cmp(&self, other: &Self) -> Ordering {
        match &self {
            TichuCall::GrandTichu(mc) => {
                if let TichuCall::GrandTichu(oc) = other {
                    mc.partial_cmp(oc).unwrap()
                } else {
                    Ordering::Less
                }
            }
            TichuCall::Tichu(mc) => {
                if let TichuCall::Tichu(oc) = other {
                    mc.partial_cmp(oc).unwrap()
                } else if let TichuCall::GrandTichu(_f) = other {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            TichuCall::NoCall => {
                if let TichuCall::NoCall = other {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
            TichuCall::NotYetPlayed => match other {
                TichuCall::NoCall => Ordering::Greater,
                TichuCall::NotYetPlayed => Ordering::Equal,
                _ => Ordering::Less,
            },
        }
    }
}

impl TichuCall {
    pub fn censor(&self) -> Self {
        match *self {
            TichuCall::Tichu(_c) => TichuCall::Tichu(0.),
            TichuCall::GrandTichu(_c) => TichuCall::GrandTichu(0.),
            x => x,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub players: [PlayerCards; 4],
    pub slash_score: i32, // slash and cross are arbitrary, just
    pub cross_score: i32, // to distinguish the teams (players know
    // about their partner)
    turn_index: u8,
    tichu_calls: [TichuCall; 4],
    pub expected_slash_points: i32,
    pub expected_cross_points: i32,
}

pub trait Player {
    /// maintaining the unseen cards is the player's perogative
    /// the bool should be true if the player seeks to call tichu.
    fn play(&mut self, own_hand: PlayerCards) -> (hand::Hand, TichuCall);
    // the bool returned is intent to bomb
    fn record_other_play(
        &mut self,
        play: &hand::Hand,
        is_partner: bool,
        calls_tichu: TichuCall,
    ) -> bool;

    ///First dealing bit
    fn see_first_8(&mut self, own_hand: [hand::Card; 8]) -> TichuCall;
    fn see_other_6(&mut self, own_hand: [hand::Card; 6]) -> TichuCall;

    ///Passing cards -- might as well pass the 14 dealt cards
    ///In all the [_; 3], assume [1] is the partner and the other 2 opponents
    ///The TichuCall `own_call` is the result of mediation.
    fn pass_to_players(
        &mut self,
        own_call: TichuCall,
        other_calls: [TichuCall; 3],
        own_hand: PlayerCards,
    ) -> [hand::Card; 3];
}

impl Game {
    pub fn new() -> Self {
        let mut shuffled: [hand::Card; 14 * 4] = [hand::Card {
            // Not sure why I have to fake a card, but woofers galore!
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
                tichu_calls: [TichuCall::NotYetPlayed; 4],
                expected_cross_points: 0,
                expected_slash_points: 0,
            }
        }
    }

    fn play_move_helper<T: Player>(
        &mut self,
        curr_player: &mut T,
        other_players: [&mut Option<&mut T>; 4],
    ) {
        let curr_hand = &self.players[self.turn_index as usize];
        if curr_hand.iter().all(Option::is_none) {
            self.turn_index = (self.turn_index + 1) % 4;
            return;
        }

        let (hand, calls_tichu) = curr_player.play(*curr_hand).clone();
        self.tichu_calls[self.turn_index as usize] =
            match self.tichu_calls[self.turn_index as usize] {
                TichuCall::NotYetPlayed => match hand.rank {
                    hand::HandType::Pass => TichuCall::NotYetPlayed,
                    _ => {
                        if let TichuCall::NotYetPlayed = calls_tichu {
                            TichuCall::NoCall
                        } else if let TichuCall::GrandTichu(c) = calls_tichu {
                            TichuCall::Tichu(c)
                        } else {
                            calls_tichu
                        }
                    }
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

    fn play_move<T: Player>(
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

    fn show_cards_to_player<T: Player>(cards: PlayerCards, player: &mut T) -> TichuCall {
        let mut first_8: [hand::Card; 8] = [hand::Card {
            value: hand::Value::Dog,
            suit: hand::Suit::Special,
        }; 8];
        for i in 0..8 {
            if let Option::Some(card) = cards[i] {
                first_8[i] = card;
            } else {
                panic!("Impossible: dealt less than 14 cards");
            }
        }
        let call_on_8 = player.see_first_8(first_8);

        let mut next_6: [hand::Card; 6] = [hand::Card {
            value: hand::Value::Dog,
            suit: hand::Suit::Special,
        }; 6];
        for i in 0..6 {
            if let Option::Some(card) = cards[i + 8] {
                next_6[i] = card;
            } else {
                panic!("Impossible: dealt less than 14 cards");
            }
        }
        let call_on_next = player.see_other_6(next_6);
        if let TichuCall::GrandTichu(_c) = call_on_8 {
            call_on_8
        } else {
            call_on_next
        }
    }

    fn swap_cards(
        card1: hand::Card,
        card2: hand::Card,
        hand1: &mut PlayerCards,
        hand2: &mut PlayerCards,
    ) {
        let mut into_1_broke = false;
        for i in 0..14 {
            if hand1[i] == Option::Some(card1) {
                hand1[i] = Option::Some(card2);
                into_1_broke = true;
                break;
            }
        }

        let mut into_2_broke = false;
        for i in 0..14 {
            if hand2[i] == Option::Some(card2) {
                hand2[i] = Option::Some(card1);
                into_2_broke = true;
                break;
            }
        }

        if !into_2_broke || !into_1_broke {
            panic!("Illegal trade");
        }
    }

    fn do_pass<T: Player>(
        &mut self,
        player1: &mut T,
        player2: &mut T,
        player3: &mut T,
        player4: &mut T,
    ) {
        let p1_pass = player1.pass_to_players(
            self.tichu_calls[0],
            [
                self.tichu_calls[1].censor(),
                self.tichu_calls[2].censor(),
                self.tichu_calls[3].censor(),
            ],
            self.players[0],
        );
        let p2_pass = player2.pass_to_players(
            self.tichu_calls[1],
            [
                self.tichu_calls[0].censor(),
                self.tichu_calls[3].censor(),
                self.tichu_calls[2].censor(),
            ],
            self.players[1],
        );
        let p3_pass = player3.pass_to_players(
            self.tichu_calls[2],
            [
                self.tichu_calls[1].censor(),
                self.tichu_calls[0].censor(),
                self.tichu_calls[3].censor(),
            ],
            self.players[2],
        );
        let p4_pass = player4.pass_to_players(
            self.tichu_calls[3],
            [
                self.tichu_calls[0].censor(),
                self.tichu_calls[1].censor(),
                self.tichu_calls[2].censor(),
            ],
            self.players[3],
        );

        {
            let (p0, p1) = self.players.split_at_mut(1);
            Game::swap_cards(p1_pass[0], p2_pass[0], &mut p0[0], &mut p1[0]);
        }
        {
            let (p0, p1) = self.players.split_at_mut(2);
            Game::swap_cards(p1_pass[1], p3_pass[1], &mut p0[0], &mut p1[0]);
        }
        {
            let (p0, p1) = self.players.split_at_mut(3);
            Game::swap_cards(p1_pass[2], p4_pass[0], &mut p0[0], &mut p1[0]);
        }
        {
            let (p0, p1) = self.players.split_at_mut(3);
            Game::swap_cards(p2_pass[1], p4_pass[1], &mut p0[1], &mut p1[0]);
        }
        {
            let (p0, p1) = self.players.split_at_mut(2);
            Game::swap_cards(p2_pass[2], p3_pass[0], &mut p0[1], &mut p1[0]);
        }
        {
            let (p0, p1) = self.players.split_at_mut(3);
            Game::swap_cards(p3_pass[2], p4_pass[2], &mut p0[2], &mut p1[0]);
        }
    }

    pub fn play_game<T: Player>(
        &mut self,
        player1: &mut T,
        player2: &mut T,
        player3: &mut T,
        player4: &mut T,
    ) {
        self.tichu_calls[0] = Game::show_cards_to_player(self.players[0], player1);
        self.tichu_calls[1] = Game::show_cards_to_player(self.players[1], player2);
        self.tichu_calls[2] = Game::show_cards_to_player(self.players[2], player3);
        self.tichu_calls[3] = Game::show_cards_to_player(self.players[3], player4);

        if self.tichu_calls[0] < self.tichu_calls[2] {
            self.tichu_calls[0] = TichuCall::NotYetPlayed;
        } else {
            self.tichu_calls[2] = TichuCall::NotYetPlayed;
        }

        if self.tichu_calls[1] < self.tichu_calls[3] {
            self.tichu_calls[2] = TichuCall::NotYetPlayed;
        } else {
            self.tichu_calls[3] = TichuCall::NotYetPlayed;
        }
    }
}
