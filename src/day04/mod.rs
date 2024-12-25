static directions: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (1, -1),
    (-1, 1),
];

pub fn solve_first(input_path: &str) -> usize {
    42
}

fn match_xmas(start: (usize, usize), direction: (i32, i32)) -> usize {
    0
}

pub fn solve_second(input_path: &str) -> usize {
    420
}

fn parse(input_path: &str) -> Vec<String> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first("C:/Dev/aoc2024/src/day04/test1.txt"), 143);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second("C:/Dev/aoc2024/src/day04/test1.txt"), 123);
    }
}
