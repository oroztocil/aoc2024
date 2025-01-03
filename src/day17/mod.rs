pub fn solve_first(input: &str) -> String {
    String::from("abraka")
}

pub fn solve_second(input: &str) -> String {
    String::from("dabra")
}

fn parse_input(input: &str) -> () {}

#[cfg(test)]
mod tests {
    use crate::utils::read_input_file;

    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first(&read_input_file("day17/test1.txt")), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second(&read_input_file("day17/test1.txt")), "dabra");
    }
}
