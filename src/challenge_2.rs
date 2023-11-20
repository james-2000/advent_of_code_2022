use std::{
    io::{BufRead, BufReader, Cursor},
    str::FromStr,
};

use crate::challenge::Challenge;

pub struct Challenge2;
#[derive(Debug, PartialEq, Eq)]
struct ParsePlayError;

enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

enum Condition {
    WIN,
    LOSS,
    DRAW,
}

impl Condition {
    pub fn from_char(char: u8) -> Condition {
        match char {
            b'X' => Condition::LOSS,
            b'Y' => Condition::DRAW,
            b'Z' => Condition::WIN,
            _ => panic!("Invalid character: {}", char as char),
        }
    }
}

impl Move {
    pub fn get_points(&self) -> usize {
        return match &self {
            Move::ROCK => 1,
            Move::PAPER => 2,
            Move::SCISSORS => 3,
        };
    }
    fn from_char(char: u8) -> Move {
        return match char {
            b'A' | b'X' => Move::ROCK,
            b'B' | b'Y' => Move::PAPER,
            b'C' | b'Z' => Move::SCISSORS,
            _ => panic!("Invalid character: {}", char),
        };
    }
}

fn get_move_outcome(their_move: &Move, my_move: &Move) -> usize {
    return match (their_move, my_move) {
        (Move::ROCK, Move::PAPER) => 6,
        (Move::ROCK, Move::SCISSORS) => 0,
        (Move::PAPER, Move::ROCK) => 0,
        (Move::PAPER, Move::SCISSORS) => 6,
        (Move::SCISSORS, Move::ROCK) => 6,
        (Move::SCISSORS, Move::PAPER) => 0,
        _ => 3,
    };
}

fn get_move_required(their_move: &Move, condition: &Condition) -> Move {
    return match their_move {
        Move::ROCK => match condition {
            Condition::WIN => Move::PAPER,
            Condition::LOSS => Move::SCISSORS,
            Condition::DRAW => Move::ROCK,
        },
        Move::PAPER => match condition {
            Condition::WIN => Move::SCISSORS,
            Condition::LOSS => Move::ROCK,
            Condition::DRAW => Move::PAPER,
        },
        Move::SCISSORS => match condition {
            Condition::WIN => Move::ROCK,
            Condition::LOSS => Move::PAPER,
            Condition::DRAW => Move::SCISSORS,
        },
    };
}

impl Challenge for Challenge2 {
    fn run(&self) {
        let mut queue = include_bytes!("../data/challenge_2.txt").to_vec();
        queue.reverse();
        let mut total_score_first = 0;
        let mut total_score_second = 0;

        while !queue.is_empty() {
            if let (Some(their), Some(_), Some(my), Some(_)) =
                (queue.pop(), queue.pop(), queue.pop(), queue.pop())
            {
                let their_move = Move::from_char(their);
                let my_move = Move::from_char(my);
                total_score_first += my_move.get_points() + get_move_outcome(&their_move, &my_move);

                let required_condition = Condition::from_char(my);
                let my_move = get_move_required(&their_move, &required_condition);
                total_score_second +=
                    my_move.get_points() + get_move_outcome(&their_move, &my_move);
            }
        }
        println!("Total Score (First Version): {}", total_score_first);
        println!("Total Score (Second Version): {}", total_score_second);
    }
}
