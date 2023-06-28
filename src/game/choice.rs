use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::io;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

impl Choice {
    // return 1 if stronger
    pub fn get_strength(&self, other: &Choice) -> i32 {
        match (self, other) {
            (Choice::Rock, Choice::Scissors) | (Choice::Rock, Choice::Lizard) => 1,
            (Choice::Scissors, Choice::Paper) | (Choice::Scissors, Choice::Lizard) => 1,
            (Choice::Paper, Choice::Rock) | (Choice::Paper, Choice::Spock) => 1,
            (Choice::Lizard, Choice::Paper) | (Choice::Lizard, Choice::Spock) => 1,
            (Choice::Spock, Choice::Rock) | (Choice::Spock, Choice::Scissors) => 1,
            (Choice::Scissors, Choice::Rock) | (Choice::Lizard, Choice::Rock) => -1,
            (Choice::Paper, Choice::Scissors) | (Choice::Lizard, Choice::Scissors) => -1,
            (Choice::Rock, Choice::Paper) | (Choice::Spock, Choice::Paper) => -1,
            (Choice::Paper, Choice::Lizard) | (Choice::Spock, Choice::Lizard) => -1,
            (Choice::Rock, Choice::Spock) | (Choice::Scissors, Choice::Spock) => -1,
            _ => 0,
        }
    }

    pub fn action(&self, other: &Choice)->String{
        match (self, other){
            (Choice::Scissors, Choice::Paper) =>"cuts".to_string(),
            (Choice::Paper, Choice::Rock) =>"covers".to_string(),
            (Choice::Rock, Choice::Lizard) =>"crushes".to_string(),
            (Choice::Lizard, Choice::Spock) =>"poisons".to_string(),
            (Choice::Spock, Choice::Scissors) =>"smashes".to_string(),
            (Choice::Scissors, Choice::Lizard) =>"decapitates".to_string(),
            (Choice::Lizard, Choice::Paper) =>"eats".to_string(),
            (Choice::Paper, Choice::Spock) =>"disproves".to_string(),
            (Choice::Spock, Choice::Rock) =>"vaporizes".to_string(),
            _ => "crushes".to_string(),
        }
    }

    pub fn ask() -> Choice {
        loop {
            println!("Please enter your choice (rock, paper, scissors, lizard, spock):");
            let mut input = String::new();

            if let Ok(_) = io::stdin().read_line(&mut input) {
                match input.trim().to_lowercase().as_str() {
                    "rock" => return Choice::Rock,
                    "paper" => return Choice::Paper,
                    "scissors" => return Choice::Scissors,
                    "lizard" => return Choice::Lizard,
                    "spock" => return Choice::Spock,
                    _ => println!("Invalid choice! Please try again."),
                }
            } else {
                println!("Error reading input. Please try again.");
            }
        }
    }
}

impl Distribution<Choice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Choice {
        match rng.gen_range(0..=4) {
            0 => Choice::Rock,
            1 => Choice::Paper,
            2 => Choice::Scissors,
            3 => Choice::Lizard,
            _ => Choice::Spock,
        }
    }
}
// get random choice
// let choice: Choice = rand::random();

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Choice::Rock => write!(f, "🪨"),
            Choice::Paper => write!(f, "📰"),
            Choice::Scissors => write!(f, "✂️"),
            Choice::Lizard => write!(f, "🦎"),
            Choice::Spock => write!(f, "🖖"),
        }
    }
}