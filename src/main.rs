mod day01;
mod day02;
mod day03;
mod day05;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: i32 = args[1].parse().expect("Invalid day number");

    match day {
        1 => {
            println!("{}", day01::solve_first());
            println!("{}", day01::solve_second());
        }
        2 => {
            println!("{}", day02::solve_first());
            println!("{}", day02::solve_second());
        },
        3 => {
            println!("{}", day03::solve_first("C:/Dev/aoc2024/src/day03/input.txt"));
            println!("{}", day03::solve_second("C:/Dev/aoc2024/src/day03/input.txt"));
        },
        5 => {
            println!("{}", day05::solve_first("C:/Dev/aoc2024/src/day05/input.txt"));
            println!("{}", day05::solve_second("C:/Dev/aoc2024/src/day05/input.txt"));
        }
        _ => {
            println!("Day not implemented");
        }
    }
}
