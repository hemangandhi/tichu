mod hand;

pub type PlayerCards = [tichu::hand::Card; 14];

pub struct Game{
    pub players: [PlayerCards; 4],
    pub slash_score: i32, // slash and cross are arbitrary, just
    pub cross_score: i32  // to distinguish the teams (players know
                          // about their partner)
}


