use std::io;

#[derive(Clone)]
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
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => panic!("invalid Shape"),
        }
    }

    fn for_outcome_against(o: &Outcome, s: &Shape) -> Shape {
        match o {
            Outcome::Win => match s {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            Outcome::Draw => s.clone(),
            Outcome::Lose => match s {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
        }
    }

    fn score_against(&self, s: &Shape) -> u32 {
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

impl Outcome {
    fn from_char(c: char) -> Outcome {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("invalid Outcome"),
        }
    }
}

fn main() {
    let mut score = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();

        let opponent = Shape::from_char(line.chars().nth(0).unwrap());
        let outcome = Outcome::from_char(line.chars().nth(2).unwrap());

        let you = Shape::for_outcome_against(&outcome, &opponent);
        score += you.score_against(&opponent) + you as u32;
    }

    println!("{}", score);
}
