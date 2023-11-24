use challenge::Challenge;
use challenge_1::Challenge1;
use challenge_2::Challenge2;
use challenge_3::Challenge3;
use challenge_4::Challenge4;
use clap::Parser;
use std::{time::Instant, vec};

mod challenge;
mod challenge_1;
mod challenge_2;
mod challenge_3;
mod challenge_4;

const CHALLENGE_OFFSET: isize = -1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    challenge: Option<usize>,
    #[arg(short, long, default_value_t = false)]
    time: bool,
}

fn main() {
    let args = Args::parse();
    let challenges: Vec<Box<dyn Challenge>> = vec![
        Box::new(Challenge1),
        Box::new(Challenge2),
        Box::new(Challenge3),
        Box::new(Challenge4),
    ];

    match args.challenge {
        Some(c) => run(&challenges, c, args.time),
        None => run_all(&challenges, args.time),
    }
}

fn run_all(challenges: &Vec<Box<dyn Challenge>>, time: bool) {
    let mut challenge_number = 1;
    let start = Instant::now();
    for challenge in challenges {
        println!("Challenge: {}", challenge_number);
        challenge_number += 1;
        if time {
            challenge.time_run()
        } else {
            challenge.run()
        }
    }

    if time {
        let duration = start.elapsed();
        println!("Total Execution time: {:?}", duration);
    }
}

fn run(challenges: &Vec<Box<dyn Challenge>>, challenge: usize, time: bool) {
    let challenge_index = (challenge as isize + CHALLENGE_OFFSET) as usize;
    let selected_challenge = challenges.get(challenge_index);
    match selected_challenge {
        Some(challenge) => {
            if time {
                challenge.time_run()
            } else {
                challenge.run()
            }
        }
        None => panic!("Invalid Challenge Number: {}", challenge),
    }
}
