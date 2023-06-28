use rand::random;
use super::choice::*;
#[derive(Debug)]
pub struct Matchup {
    player1: String,
    player2: String,
    // result: String,
}


impl Matchup{
    pub fn new(player1: String, player2: String) -> Self {
        Matchup {
            player1,
            player2,
            // result: String::new(),
        }
    }

    pub fn play(&self) -> String{
        let player_1_move: Choice = random();
        let player_2_move: Choice = random();

        let outcome = player_1_move.get_strength(&player_2_move);

        match outcome {
            1 => println!("Player {} wins!", self.player1),
            -1 => println!("Player {} wins!", self.player2),
            _ => println!("It's a tie!"),
        };
        
        self.player1.clone()
    }
}