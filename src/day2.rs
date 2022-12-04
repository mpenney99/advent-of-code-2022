use crate::utils::*;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

const SCORE_LOSE: u32 = 0;
const SCORE_DRAW: u32 = 3;
const SCORE_WIN: u32 = 3;
const SHAPES: [Shape; 3] = [Shape::Rock, Shape::Paper, Shape::Scissors];


fn shape_score(shape: &Shape) -> u32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    }
}

fn outcome_score(a: &Shape, b: &Shape) -> u32 {
    match a {
        Shape::Rock => match b {
            Shape::Scissors => SCORE_WIN,
            Shape::Rock => SCORE_DRAW,
            Shape::Paper => SCORE_LOSE,
        },
        Shape::Paper => match b {
            Shape::Rock => SCORE_WIN,
            Shape::Paper => SCORE_DRAW,
            Shape::Scissors => SCORE_LOSE,
        },
        Shape::Scissors => match b {
            Shape::Paper => SCORE_WIN,
            Shape::Scissors => SCORE_DRAW,
            Shape::Rock => SCORE_LOSE,
        },
    }
}

fn round_score(player_shape: Shape, opp_shape: Shape) -> u32 {
    shape_score(&player_shape) + outcome_score(&player_shape, &opp_shape)
}

fn opponent_shape(input: &str) -> Shape {
    match input {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!()
    }
}

fn player_shape(opp_shape: &Shape, input: &str) -> Shape {
    SHAPES.into_iter().find(|shape| {
        let score = outcome_score(shape, opp_shape);
        match input {
            "X" => score == SCORE_LOSE,
            "Y" => score == SCORE_DRAW,
            "Z" => score == SCORE_WIN,
            _ => panic!()
        }
    }).unwrap()
}

pub fn problem2() {
    let mut total_score: u32 = 0;

    read_lines("./src/day2_input")
        .for_each(|line| {
            if let Ok(text) = line {
                let mut parts = text.split(" ");
                let opp_shape: Shape = opponent_shape(parts.next().unwrap());
                let player_shape: Shape = player_shape(&opp_shape, parts.next().unwrap());
                total_score += round_score(player_shape, opp_shape)
            }
        });

    println!("{}", total_score);
}
