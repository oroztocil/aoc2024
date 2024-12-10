use regex::Regex;
use std::fs;

pub fn solve_first(input_path: &str) -> usize {
    find_and_sum_muls(&load_input(input_path))
}

pub fn solve_second(input_path: &str) -> usize {
    load_input(&input_path)
        .split("do()")
        .map(|part| part.split_once("don't()").map_or(part, |(first, _)| first))
        .map(|part| find_and_sum_muls(part))
        .sum()
}

fn find_and_sum_muls(input: &str) -> usize {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total: usize = 0;

    for captures in regex.captures_iter(&input) {
        let a: usize = captures[1].parse().unwrap();
        let b: usize = captures[2].parse().unwrap();
        total += a * b;
    }

    total
}

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Error reading file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first("C:/Dev/aoc2024/src/day03/test1.txt"), 161);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second("C:/Dev/aoc2024/src/day03/test2.txt"), 48);
    }
}