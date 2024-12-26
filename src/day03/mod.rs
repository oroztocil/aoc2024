use regex::Regex;

pub fn solve_first(input: &str) -> usize {
    find_and_sum_muls(&input)
}

pub fn solve_second(input: &str) -> usize {
    input
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

#[cfg(test)]
mod tests {
    use crate::utils::read_input_file;

    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first(&read_input_file("day03/test1.txt")), 161);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second(&read_input_file("day03/test2.txt")), 48);
    }
}
