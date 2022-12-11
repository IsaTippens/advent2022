use std::env;
use advent2022::init_solutions;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        welcome();
        return;
    }
    let day = args[1].parse::<usize>().unwrap() - 1;
    let d = init_solutions();
    d.SelectDay(day);
}

fn welcome() {
    println!("HI!")
}
