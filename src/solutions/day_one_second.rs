use crate::advent::Advent;
use std::fs::File;
use std::io::{self, BufRead};

pub struct DayOneSecond {

}

impl DayOneSecond {
    fn push_largest(&self, top: &mut Vec<usize>, mut num: usize) {
        for i in 0..top.len() {
            if top[i] < num {
                let temp = top[i];
                top[i] = num;
                num = temp;
            }
        }
    }
}

impl Advent for DayOneSecond {
    fn Start(&self) {
        println!("Day 1 Puzzle 2");

        let input = File::open("inputs/1_1.txt").expect("Could not open 1_1.txt");
        let lines = io::BufReader::new(input).lines();

        let mut largest: Vec<usize> = vec![0, 0, 0];
        let mut current: usize = 0;
        for line in lines {
            if let Ok(l) = line {
                if l.is_empty() {
                    self.push_largest(&mut largest, current);
                    current = 0;
                    continue;
                }
                let calories = l.parse::<usize>().expect("Could not parse line");
                current += calories;
            }
        }

        let mut sum = 0;
        for i in 0..largest.len() {
            sum += largest[i];
        }
        println!("Largest sum: {:?}", sum);   

    }
}