mod matchup;
mod player;

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
        let matchups = Game::create_matchups
        (players.keys().cloned().collect());
        Self {
            rounds,
            players,
            matchups,
        }
    }

    pub fn start(&mut self) {
        // Implement the game logic here
        // Simulate rounds, track matchups, determine winners, etc.
        // Use self.players and self.rounds to access the player count and round count
        // Display the results at the end
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
