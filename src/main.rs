mod game;

use game::Game;
use std::io;

fn main() {
    println!("Welcome player, and let the journey begin!");

    // Read player count from console
    let player_count = read_input("Enter the number of players (1-9):", 1, 9);

    // Read round count from console
    let round_count = read_input("Enter the number of rounds (1-5):", 1, 5);

    // define new game
    let mut game = Game::new(round_count, player_count);
    game.start()
}

fn read_input(prompt: &str, min: u8, max: u8) -> u8 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) if value >= min && value <= max => {
                return value;
            }
            _ => println!(
                "Invalid input. Please enter a number between {} and {}.",
                min, max
            ),
        };
    }
}
