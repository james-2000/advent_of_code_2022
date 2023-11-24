use std::io::{BufRead, BufReader, Cursor};

use crate::challenge::Challenge;

pub struct Challenge4;

fn get_range(string: &str) -> (i32, i32) {
    let split: Vec<_> = string.split('-').collect();
    let number_one = split.get(0).unwrap().parse::<i32>().unwrap();
    let number_two = split.get(1).unwrap().parse::<i32>().unwrap();
    (number_one, number_two)
}

impl Challenge for Challenge4 {
    fn run(&self) {
        let data = include_bytes!("../data/challenge_4.txt");
        let buffer = BufReader::new(Cursor::new(data));
        let mut grand_total_total = 0;
        let mut grand_total_overlap = 0;

        buffer.lines().for_each(|line| {
            if let Ok(line) = line {
                let ranges: Vec<_> = line.split(',').collect();
                let range_one = get_range(ranges.get(0).unwrap());
                let range_two = get_range(ranges.get(1).unwrap());

                if (range_two.0 >= range_one.0 && range_two.0 <= range_one.1)
                    || (range_two.1 >= range_one.0 && range_two.1 <= range_one.1)
                    || (range_one.0 >= range_two.0 && range_one.0 <= range_two.1)
                    || (range_one.1 >= range_two.0 && range_one.1 <= range_two.1)
                {
                    grand_total_overlap += 1;
                }

                println!(
                    "Ranges: {}-{} and {}-{}",
                    range_one.0, range_one.1, range_two.0, range_two.1
                );

                if (range_one.0 <= range_two.0 && range_one.1 >= range_two.1)
                    || (range_two.0 <= range_one.0 && range_two.1 >= range_one.1)
                {
                    grand_total_total += 1;
                }
            }
        });
        println!("Grand Total: {}", grand_total_total);
        println!("Grand Total Overlap: {}", grand_total_overlap);
    }
}
