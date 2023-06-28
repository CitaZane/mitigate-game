// mod player;
// mod matchup;
mod player;
use player::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Game {
    pub players: Vec<Player>,
    rounds: u8,

}

impl Game {
    pub fn new(rounds:u8, player_count:u8)->Self{
        let players = Game::create_players(player_count);

        Self{
            rounds,
            players,
        }
    }

    pub fn start(&self){
        // Implement the game logic here
        // Simulate rounds, track matchups, determine winners, etc.
        // Use self.players and self.rounds to access the player count and round count
        // Display the results at the end
    }

    fn create_players(player_count:u8)-> Vec<Player> {
        let mut players = Vec::new();

        let names = [
            "Sheldon", "Leonard", "Penny", "Howard", "Raj",
            "Amy", "Bernadette", "Stuart", "Wil Wheaton"
        ];
        let mut funny_names = [
            "Crazy Cat", "Bazinga", "Rock Star", "Super Nerd", "The Guru",
            "Hilarious Hero", "Lizard Lover", "The Jester", "Mastermind"
        ];
        // shuffle names
        let mut rng = thread_rng();
        funny_names.shuffle(&mut rng);
        // add computer players
        for i in 0..player_count as usize{
            let name = format!("{} {}", funny_names[i], names[i]);
            let player = Player::computer(name);
            players.push(player);
        }
        // add human player
        let player = Player::human();
        players.push(player);

        players
    }

}