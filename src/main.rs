use utils::OUT_DIR;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: i32 = args[1].parse().expect("Invalid day number");

    match day {
        1 => {
            println!("{}", day01::solve_first(&OUT_DIR.join("day01/input.txt")));
            println!("{}", day01::solve_second(&OUT_DIR.join("day01/input.txt")));
        }
        2 => {
            println!("{}", day02::solve_first(&OUT_DIR.join("day02/input.txt")));
            println!("{}", day02::solve_second(&OUT_DIR.join("day02/input.txt")));
        },
        3 => {
            println!("{}", day03::solve_first(&OUT_DIR.join("day03/input.txt")));
            println!("{}", day03::solve_second(&OUT_DIR.join("day03/input.txt")));
        },
        4 => {
            println!("{}", day04::solve_first(&OUT_DIR.join("day04/input.txt")));
            println!("{}", day04::solve_second(&OUT_DIR.join("day04/input.txt")));
        },
        5 => {
            println!("{}", day05::solve_first(&OUT_DIR.join("day05/input.txt")));
            println!("{}", day05::solve_second(&OUT_DIR.join("day05/input.txt")));
        }
        _ => {
            println!("Day not implemented");
        }
    }
}
