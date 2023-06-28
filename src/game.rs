mod choice;
mod matchup;
mod player;

use choice::*;
use matchup::*;
use player::*;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

pub struct Game {
    pub players: HashMap<String, Player>,
    rounds: u8,
    pub matchups: Vec<Matchup>,
}

impl Game {
    pub fn new(rounds: u8, player_count: u8) -> Self {
        let players = Game::create_players(player_count);
        let matchups = Game::create_matchups(players.keys().cloned().collect());
        Self {
            rounds,
            players,
            matchups,
        }
    }

    pub fn start(&mut self) {
        // Simulate rounds and update victory counts
        for round in 1..=self.rounds {
            println!("〰️〰️〰️〰️〰️〰️ ROUND {round} 〰️〰️〰️〰️〰️〰️");

            for matchup in &self.matchups {
                let player_1_human = self.players.get(&matchup.player1).unwrap().is_human;
                let player_2_human = self.players.get(&matchup.player2).unwrap().is_human;

                let winner_name = matchup.play(player_1_human, player_2_human);

                // increase score
                if let Some(winner) = self.players.get_mut(&winner_name) {
                    winner.victory_count += 1;
                }
            }
        }
        self.print_scores();
        self.find_winner();
    }
    fn print_scores(&self) {
        println!("〰️〰️〰️〰️〰️〰️ RESULTS 〰️〰️〰️〰️〰️〰️");
        // Print scores for all players
        for player in self.players.values() {
            println!("Player: {}  Victory count: {}", player.name, player.victory_count);
        }
    }
    fn find_winner(&self) {
        let mut highest_score = 0;
        let mut winners: Vec<&str> = Vec::new();

        for (name, player) in &self.players {
            if player.victory_count > highest_score {
                highest_score = player.victory_count;
                winners.clear();
                winners.push(name);
            }else if player.victory_count == highest_score {
                winners.push(name);
            }
        }
        if winners.len() == 1 {
            println!("🏆 🏆 🏆  Winner: {} with score {} 🏆 🏆 🏆 ", winners[0], highest_score);
        } else {
            println!("🏆 🏆 🏆 It's a tie! Winners with score {}: {:?} 🏆 🏆 🏆", highest_score, winners);
        }
    }

    fn create_players(player_count: u8) -> HashMap<String, Player> {
        let mut players = HashMap::new();

        let names = [
            "Sheldon",
            "Leonard",
            "Penny",
            "Howard",
            "Raj",
            "Amy",
            "Bernadette",
            "Stuart",
            "Wil Wheaton",
        ];
        let mut funny_names = [
            "Crazy Cat",
            "Bazinga",
            "Rock Star",
            "Super Nerd",
            "The Guru",
            "Hilarious Hero",
            "Lizard Lover",
            "The Jester",
            "Mastermind",
        ];
        // shuffle names
        let mut rng = thread_rng();
        funny_names.shuffle(&mut rng);
        // add computer players
        for i in 0..player_count as usize {
            let name = format!("{} {}", funny_names[i], names[i]);
            let player = Player::computer(name);
            players.insert(player.name.clone(), player);
        }
        // add human player
        let player = Player::human();
        players.insert(player.name.clone(), player);

        players
    }

    fn create_matchups(players: Vec<String>) -> Vec<Matchup> {
        let mut matchups = Vec::new();
        let player_count = players.len();

        for i in 0..player_count - 1 {
            for j in i + 1..player_count {
                let player1 = &players[i];
                let player2 = &players[j];
                let matchup = Matchup::new(player1.clone(), player2.clone());
                matchups.push(matchup);
            }
        }

        matchups
    }
}
