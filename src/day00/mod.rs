pub fn solve_first(input: &str) -> usize {
    42
}

pub fn solve_second(input: &str) -> usize {
    420
}

fn parse_input(input: &str) -> () {}

#[cfg(test)]
mod tests {
    use crate::utils::read_input_file;

    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first(&read_input_file("day0N/test1.txt")), 42);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second(&read_input_file("day0N/test1.txt")), 420);
    }
}
