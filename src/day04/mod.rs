#![allow(dead_code)]
#![allow(unused_variables)]

static DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (1, -1),
    (-1, 1),
];

pub fn solve_first(input: &str) -> usize {
    42
}

fn match_xmas(start: (usize, usize), direction: (i32, i32)) -> usize {
    0
}

pub fn solve_second(input: &str) -> usize {
    420
}

// fn parse(input_path: &str) -> Vec<String> {}

#[cfg(test)]
mod tests {
    use crate::utils::read_input_file;

    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first(&read_input_file("day04/test1.txt")), 42);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second(&read_input_file("day04/test1.txt")), 420);
    }
}

