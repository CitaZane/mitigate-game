use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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

// let choice: Choice = rand::random();