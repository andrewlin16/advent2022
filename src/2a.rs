use std::io;

enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl Shape {
    fn from_char(c: char) -> Shape {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("invalid Shape"),
        }
    }

    fn score_against(&self, s: Shape) -> u32 {
        (match self {
            Shape::Rock => match s {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Lose,
                Shape::Scissors => Outcome::Win,
            },
            Shape::Paper => match s {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Lose,
            },
            Shape::Scissors => match s {
                Shape::Rock => Outcome::Lose,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Draw,
            },
        }) as u32
    }
}

fn main() {
    let mut score = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();

        let opponent = Shape::from_char(
            line.chars()
                .nth(0)
                .expect("strategy guide should have 0th character for opponent's play"),
        );
        let you = Shape::from_char(
            line.chars()
                .nth(2)
                .expect("strategy guide should have 2nd character for own play"),
        );

        score += you.score_against(opponent) + you as u32;
    }

    println!("{}", score);
}
