use super::choice::*;
use rand::random;
use std::thread;
use std::time::Duration;
#[derive(Debug)]
pub struct Matchup {
    pub player1: String,
    pub player2: String,
    // result: String,
}

impl Matchup {
    pub fn new(player1: String, player2: String) -> Self {
        Matchup {
            player1,
            player2,
            // result: String::new(),
        }
    }

    pub fn play(&self, player_1_human: bool, player_2_human: bool) -> String {
        // get choices
        let player_1_move= Matchup::make_a_choice(player_1_human);
        let player_2_move= Matchup::make_a_choice(player_2_human);

        println!("\n{} - {}", self.player1, player_1_move);
        println!("{} - {}", self.player2, player_2_move);
        // calculate outcome
        let outcome = player_1_move.get_strength(&player_2_move);

        if outcome == 1 {
            println!(
                "{} {} {}",
                player_1_move,
                player_1_move.action(&player_2_move),
                player_2_move
            );
            return self.player1.clone();
        } else if outcome == -1 {
            println!(
                "{} {} {}",
                player_2_move,
                player_2_move.action(&player_1_move),
                player_1_move
            );
            return self.player2.clone();
        } else {
            println!("It's a tie!");
            return self.play(player_1_human, player_2_human);
        }
    }

    fn make_a_choice(player_is_human:bool)->Choice{
        if player_is_human {
            return Choice::ask()
        } else {
            // delay computer choice for more readable game
            thread::sleep(Duration::from_millis(2000));
            return random()
        };
    }
}
