#[derive(Debug)]
pub struct Matchup {
    player1: String,
    player2: String,
    result: String,
}


impl Matchup{
    pub fn new(player1: String, player2: String) -> Self {
        Matchup {
            player1,
            player2,
            result: String::new(),
        }
    }
}