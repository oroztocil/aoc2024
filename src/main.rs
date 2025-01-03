use utils::read_input_file;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day17;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: i32 = args[1].parse().expect("Invalid day number");

    match day {
        1 => {
            println!(
                "{}",
                day01::solve_first(&read_input_file("day01/input.txt"))
            );
            println!(
                "{}",
                day01::solve_second(&read_input_file("day01/input.txt"))
            );
        }
        2 => {
            println!(
                "{}",
                day02::solve_first(&read_input_file("day02/input.txt"))
            );
            println!(
                "{}",
                day02::solve_second(&read_input_file("day02/input.txt"))
            );
        }
        3 => {
            println!(
                "{}",
                day03::solve_first(&read_input_file("day03/input.txt"))
            );
            println!(
                "{}",
                day03::solve_second(&read_input_file("day03/input.txt"))
            );
        }
        4 => {
            println!(
                "{}",
                day04::solve_first(&read_input_file("day04/input.txt"))
            );
            println!(
                "{}",
                day04::solve_second(&read_input_file("day04/input.txt"))
            );
        }
        5 => {
            println!(
                "{}",
                day05::solve_first(&read_input_file("day05/input.txt"))
            );
            println!(
                "{}",
                day05::solve_second(&read_input_file("day05/input.txt"))
            );
        }
        6 => {
            println!(
                "{}",
                day06::solve_first(&read_input_file("day06/input.txt"))
            );
            println!(
                "{}",
                day06::solve_second(&read_input_file("day06/input.txt"))
            );
        }
        7 => {
            println!(
                "{}",
                day07::solve_first(&read_input_file("day07/input.txt"))
            );
            println!(
                "{}",
                day07::solve_second(&read_input_file("day07/input.txt"))
            );
        }
        17 => {
            println!(
                "{}",
                day17::solve_first(&read_input_file("day17/input.txt"))
            );
            println!(
                "{}",
                day17::solve_second(&read_input_file("day17/input.txt"))
            );
        }
        _ => {
            println!("Day not implemented");
        }
    }
}
