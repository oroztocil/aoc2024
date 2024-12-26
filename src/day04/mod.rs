#![allow(dead_code)]
#![allow(unused_variables)]

use std::path::Path;

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

pub fn solve_first(input_path: &Path) -> usize {
    42
}

fn match_xmas(start: (usize, usize), direction: (i32, i32)) -> usize {
    0
}

pub fn solve_second(input_path: &Path) -> usize {
    420
}

// fn parse(input_path: &str) -> Vec<String> {}

// #[cfg(test)]
// mod tests {
//     use crate::utils::OUT_DIR;

//     use super::*;

//     #[test]
//     fn test_first() {
//         assert_eq!(solve_first(&OUT_DIR.join("day04/test1.txt")), 143);
//     }

//     #[test]
//     fn test_second() {
//         assert_eq!(solve_second(&OUT_DIR.join("day04/test1.txt")), 123);
//     }
// }
