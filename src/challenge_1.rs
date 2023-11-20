use std::io::{BufRead, BufReader, Cursor};

use crate::challenge::Challenge;

pub struct Challenge1;

impl Challenge for Challenge1 {
    fn run(&self) {
        let data = include_bytes!("../data/challenge_1.txt");
        let buffer = BufReader::new(Cursor::new(data));
        let mut three_largest_seen = [0, 0, 0];
        let mut current_tally = 0;

        for line in buffer.lines() {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        for largest_seen in three_largest_seen.iter_mut() {
                            if *largest_seen < current_tally {
                                let old_value = *largest_seen;
                                *largest_seen = current_tally;
                                current_tally = old_value;
                            };
                        }
                        current_tally = 0;
                        continue;
                    }
                    current_tally += line.parse::<i32>().expect("Line is not a number!");
                }
                Err(e) => panic!("Error: {}", e),
            }
        }
        let mut grand_total = 0;
        for value in three_largest_seen {
            grand_total += value;
            println!("{}", value);
        }
        println!("Grand Total: {}", grand_total);
    }
}
