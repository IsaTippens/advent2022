use crate::advent::Advent;
use std::fs::File;
use std::io::{self, BufRead};

pub struct DayOne {

}

impl Advent for DayOne {
    fn Start(&self) {
        println!("Day 1 Puzzle 1");
        let input = File::open("inputs/1_1.txt").expect("Could not open 1_1.txt");
        let lines = io::BufReader::new(input).lines();

        let mut largest: usize = 0;
        let mut current: usize = 0;
        for line in lines {
            if let Ok(l) = line {
                if l.is_empty() {
                    current = 0;
                    continue;
                }
                let calories = l.parse::<usize>().expect("Could not parse line");
                current += calories;
            }
            if current > largest {
                largest = current;
            }
        }

        println!("Largest sum: {:?}", largest);   

    }
}