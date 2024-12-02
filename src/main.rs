mod day01;
mod day02;
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
        }
        _ => {
            println!("Day not implemented");
        }
    }
}
