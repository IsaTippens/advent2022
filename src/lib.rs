mod advent;
use advent::Advent;
mod solutions;
use solutions::*;

pub struct Days {
    solutions: Vec<Box<dyn Advent>>
}

// initialse Days
pub fn init_solutions() -> Days {
    let mut solutions = Vec::new();
    solutions.push(Box::new(day_one_first::DayOne{}) as Box<dyn Advent>);
    solutions.push(Box::new(day_one_second::DayOneSecond{}) as Box<dyn Advent>);
    Days{
        solutions: solutions
    }
}

impl Days {
    pub fn SelectDay(&self, day: usize) {
        if day > self.solutions.len() || day < 0{
            print!("Day does not exist");
            return;
        }
        self.solutions[day].Start();
    }
}
