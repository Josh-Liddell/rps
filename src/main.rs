use rand::Rng;
use std::io;

struct Rps {
    rounds: u8,
    uwins: u8,
    cwins: u8,
}

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    User,
    Computer,
    Tie,
}

impl Rps {
    fn new() -> Self {
        println!("Welcome to rock paper scissors!");
        println!("How many rounds would you like to play?");

        let rounds = Self::get_rounds();

        Self {
            rounds,
            uwins: 0,
            cwins: 0,
        }
    }

    fn get_rounds() -> u8 {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Enter a valid number");
                    continue;
                }
            };
            if input != 0 {
                break input;
            } else {
                println!("Has to be more than 0 rounds");
            }
        }

    }

    fn start(&mut self) {
        println!("Lets Begin!");

        // 1. Get user choice
        // 2. Get computer choice
        // 3. Get the outcome and do what is necessary for that outcome

        for round in 1..=self.rounds {
            println!("Round {round}!");
            println!("Make your guess: ");

            let user = self.uchoice();
            let computer = self.cchoice();

            println!("You chose {:?} and computer chose {:?}", user, computer);

            match self.get_winner(user, computer) {
                Outcome::User => {
                    println!("User won!");
                    self.uwins += 1;
                }
                Outcome::Computer => {
                    println!("Computer won!");
                    self.cwins += 1;
                }
                Outcome::Tie => {
                    println!("Tie!");
                }
            }
        }

        println!("\nGAME OVER!");
        println!(
            "User won {} games and computer won {} games",
            self.uwins, self.cwins
        );
        if self.uwins > self.cwins {
            println!("User won the game!");
        } else if self.cwins > self.uwins {
            println!("Computer won the game!");
        } else {
            println!("It was an overall tie!");
        }

        println!("\nThanks for playing!");
    }

    fn cchoice(&self) -> Choice {
        match rand::rng().random_range(1..=3) {
            1 => Choice::Rock,
            2 => Choice::Paper,
            _ => Choice::Scissors,
        }
    }

    fn uchoice(&self) -> Choice {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let choice = match input.trim() {
                "r" => Choice::Rock,
                "p" => Choice::Paper,
                "s" => Choice::Scissors,
                _ => {
                    println!("Enter a valid option (r, p, or s)");
                    continue;
                }
            };
            break choice;
        }
    }

    fn get_winner(&self, user: Choice, com: Choice) -> Outcome {
        use Choice::*;
        match (user, com) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Outcome::User,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Outcome::Tie,
            _ => Outcome::Computer,
        }
    }
}

fn main() {
    let mut game = Rps::new();
    game.start();
}
