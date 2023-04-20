use rand::Rng;
use std::io;

// I want to define an enum of possible choices
#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

fn get_player_choice() -> Choice {
    loop {
        println!("Please choose your weapon");
        println!("1. Rock");
        println!("2. Paper");
        println!("3. Scissors");

        let mut player_choice = String::new();
        io::stdin()
            .read_line(&mut player_choice)
            .expect("Failed to read line");

        match player_choice.trim() {
            "1" => break Choice::Rock,
            "2" => break Choice::Paper,
            "3" => break Choice::Scissors,
            _ => println!("Please choose a valid option"),
        }
    }
}

fn get_computer_choice() -> Choice {
    let computer_choice = rand::thread_rng().gen_range(1..=3);
    match computer_choice {
        1 => Choice::Rock,
        2 => Choice::Paper,
        _ => Choice::Scissors,
    }
}

fn get_game_outcome(player_choice: Choice, computer_choice: Choice) -> GameResult {
    match player_choice {
        Choice::Rock => match computer_choice {
            Choice::Rock => GameResult::Draw,
            Choice::Paper => GameResult::Lose,
            Choice::Scissors => GameResult::Win,
        },
        Choice::Paper => match computer_choice {
            Choice::Rock => GameResult::Win,
            Choice::Paper => GameResult::Draw,
            Choice::Scissors => GameResult::Lose,
        },
        Choice::Scissors => match computer_choice {
            Choice::Rock => GameResult::Lose,
            Choice::Paper => GameResult::Win,
            Choice::Scissors => GameResult::Draw,
        },
    }
}

struct Scoreboard {
    player_score: i32,
    computer_score: i32,
}

impl Scoreboard {
    fn new() -> Self {
        Self {
            player_score: 0,
            computer_score: 0,
        }
    }

    fn update_score(&mut self, game_outcome: GameResult) {
        match game_outcome {
            GameResult::Win => self.player_score += 1,
            GameResult::Lose => self.computer_score += 1,
            GameResult::Draw => (),
        }
    }

    fn print_score(&self) {
        println!("Player score: {}", self.player_score);
        println!("Computer score: {}", self.computer_score);
    }
}

fn main() {
    let mut scoreboard = Scoreboard::new();
    let player_choice = get_player_choice();
    let computer_choice = get_computer_choice();
    println!("You chose {:?}", player_choice);
    println!("The computer chose {:?}", computer_choice);
    let game_outcome = get_game_outcome(player_choice, computer_choice);
    scoreboard.update_score(game_outcome);
    scoreboard.print_score();
}
