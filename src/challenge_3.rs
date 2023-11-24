use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Cursor},
};

use crate::challenge::Challenge;

pub struct Challenge3;

const LOWER_CHAR_OFFSET: u8 = 96;
const UPPER_CHAR_OFFSET: u8 = 38;

fn get_priority(char: &&u8) -> usize {
    let char = *char;
    if *char >= b'a' && *char <= b'z' {
        return (*char - LOWER_CHAR_OFFSET).into();
    } else {
        return (*char - UPPER_CHAR_OFFSET).into();
    }
}

impl Challenge for Challenge3 {
    fn run(&self) {
        const GROUP_INDEX_LEN: i32 = 3;
        let data = include_bytes!("../data/challenge_3.txt");
        let buffer = BufReader::new(Cursor::new(data));
        let mut grand_total: usize = 0;
        let mut badge_total: usize = 0;
        let mut group_index = 0;
        let mut badge_vector: Vec<u8> = Vec::new();

        for line in buffer.lines() {
            match line {
                Ok(line) => {
                    let mid_point = line.len() / 2;
                    let (first, second) = line.split_at(mid_point);
                    let mut common = Vec::from_iter(
                        second
                            .as_bytes()
                            .iter()
                            .filter(|c| first.contains(**c as char)),
                    );
                    common.sort();
                    common.dedup();
                    grand_total += common.iter().map(get_priority).sum::<usize>();

                    match group_index {
                        GROUP_INDEX_LEN => {
                            group_index = 0;
                            badge_vector = line.as_bytes().to_owned().to_vec();
                        }
                        _ => {
                            badge_vector = badge_vector
                                .into_iter()
                                .filter(|x| line.contains(*x as char))
                                .collect();
                            if group_index == GROUP_INDEX_LEN - 1 {
                                badge_total +=
                                    badge_vector.iter().map(|x| get_priority(&x)).sum::<usize>();
                            }
                        }
                    };
                    group_index += 1;
                }
                Err(e) => panic!("Error: {}", e),
            }
        }
        println!("Grand Total: {}", grand_total);
        println!("Badge Total: {}", badge_total);
    }
}
