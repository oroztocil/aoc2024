#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Debug;

use regex::Regex;

use crate::utils::grid::Coords;

pub fn solve_first(input: &str) -> usize {
    let codes = parse_input(input);
    let result = codes
        .iter()
        .map(|(chars, num_value)| {
            let cost = compute_cost(&chars);
            cost * num_value
        })
        .sum();

    result
}

fn compute_cost(chars: &Vec<char>) -> usize {
    42
    // let mut current_char = 'A';
    // let mut numpad_moves: Vec<Move> = Vec::new();

    // for next_char in chars {
    //     numpad_moves.append(&mut get_numpad_moves(current_char, *next_char));
    //     numpad_moves.push(Move::Activate);
    //     current_char = *next_char;
    // }

    // let mut current_arrow = Arrow::Activate;
    // let mut arrowpad_moves: Vec<Move> = Vec::new();

    // for next_arrow in numpad_moves {
    //     arrowpad_moves.append(&mut &mut get_arrowpad_moves(
    //         current_arrow,
    //         next_arrow.clone(),
    //     ));
    //     arrowpad_moves.push(Move::Activate);
    //     current_arrow = next_arrow;
    // }

    // current_arrow = Arrow::Activate;
    // let mut arrowpad2_moves: Vec<Move> = Vec::new();

    // for next_arrow in arrowpad_moves {
    //     arrowpad2_moves.append(&mut &mut get_arrowpad_moves(
    //         current_arrow,
    //         next_arrow.clone(),
    //     ));
    //     arrowpad2_moves.push(Move::Activate);
    //     current_arrow = next_arrow;
    // }

    // println!("{:?}", arrowpad2_moves);

    // arrowpad2_moves.len()
}

#[derive(PartialEq, Clone)]
enum Move {
    Left,
    Right,
    Up,
    Down,
    Activate,
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Move::Left => '<',
            Move::Right => '>',
            Move::Up => '^',
            Move::Down => 'v',
            Move::Activate => 'A',
        };

        write!(f, "{c}")
    }
}

type Arrow = Move;

fn numpad_char_to_coords(c: char) -> Coords {
    match c {
        '7' => Coords { x: 0, y: 0 },
        '8' => Coords { x: 1, y: 0 },
        '9' => Coords { x: 2, y: 0 },
        '4' => Coords { x: 0, y: 1 },
        '5' => Coords { x: 1, y: 1 },
        '6' => Coords { x: 2, y: 1 },
        '1' => Coords { x: 0, y: 2 },
        '2' => Coords { x: 1, y: 2 },
        '3' => Coords { x: 2, y: 2 },
        '0' => Coords { x: 1, y: 3 },
        'A' => Coords { x: 2, y: 3 },
        _ => panic!("Unsupported char"),
    }
}

fn arrowpad_move_to_coords(m: Arrow) -> Coords {
    match m {
        Arrow::Left => Coords { x: 0, y: 1 },
        Arrow::Right => Coords { x: 2, y: 1 },
        Arrow::Up => Coords { x: 1, y: 0 },
        Arrow::Down => Coords { x: 1, y: 1 },
        Arrow::Activate => Coords { x: 2, y: 0 },
    }
}

#[derive(Debug, PartialEq, Clone)]
enum MoveSearchResult {
    OneWay(Vec<Move>),
    TwoWays(Vec<Move>, Vec<Move>),
}

fn get_numpad_moves(src: char, dst: char) -> MoveSearchResult {
    let (src_coords, dst_coords) = (numpad_char_to_coords(src), numpad_char_to_coords(dst));
    let delta_x = dst_coords.x - src_coords.x;
    let delta_y = dst_coords.y - src_coords.y;
    let move_x = if delta_x > 0 { Move::Right } else { Move::Left };
    let move_y = if delta_y > 0 { Move::Down } else { Move::Up };

    // Idea:
    // Making all moves along one axis, then making all moves along the other axis is
    // more optimal than switching directions multiple times.
    // However, we want to try both orders of axes.
    // (Because my brain hurts when trying to analyze which one si better in advance.)

    if src_coords.x == dst_coords.x {
        let mut moves: Vec<Move> = Vec::new();

        for _ in 0..delta_y.abs() {
            moves.push(move_y.clone());
        }

        MoveSearchResult::OneWay(moves)
    } else if src_coords.y == dst_coords.y {
        let mut moves: Vec<Move> = Vec::new();

        for _ in 0..delta_x.abs() {
            moves.push(move_x.clone());
        }

        MoveSearchResult::OneWay(moves)
    }
    // Case 1: If we are on O or A and going to the leftmost column, we want to move up first, then do the x-axis moves.
    else if src_coords.y == 3 && dst_coords.x == 0 {
        let mut moves: Vec<Move> = Vec::new();

        for _ in 0..delta_y.abs() {
            moves.push(move_y.clone());
        }

        for _ in 0..delta_x.abs() {
            moves.push(move_x.clone());
        }

        MoveSearchResult::OneWay(moves)
    }
    // Case 2: If we are in the leftmost column and going to 0 or A, we want to move right first, then do the y-axis moves.
    else if src_coords.x == 0 && dst_coords.y == 3 {
        let mut moves: Vec<Move> = Vec::new();

        for _ in 0..delta_x.abs() {
            moves.push(move_x.clone());
        }

        for _ in 0..delta_y.abs() {
            moves.push(move_y.clone());
        }

        MoveSearchResult::OneWay(moves)
    }
    // Case 3: Otherwise we return both possible orders.
    else {
        let mut x_then_y: Vec<Move> = Vec::new();

        for _ in 0..delta_x.abs() {
            x_then_y.push(move_x.clone());
        }

        for _ in 0..delta_y.abs() {
            x_then_y.push(move_y.clone());
        }

        let mut y_then_x: Vec<Move> = Vec::new();

        for _ in 0..delta_y.abs() {
            y_then_x.push(move_y.clone());
        }

        for _ in 0..delta_x.abs() {
            y_then_x.push(move_x.clone());
        }

        MoveSearchResult::TwoWays(x_then_y, y_then_x)
    }
}

fn get_arrowpad_moves(src: Arrow, dst: Arrow) -> Vec<Move> {
    let (src_coords, dst_coords) = (arrowpad_move_to_coords(src), arrowpad_move_to_coords(dst));
    let delta_x = dst_coords.x - src_coords.x;
    let delta_y = dst_coords.y - src_coords.y;
    let move_x = if delta_x > 0 { Move::Right } else { Move::Left };
    let move_y = if delta_y > 0 { Move::Down } else { Move::Up };

    let mut moves: Vec<Move> = Vec::new();

    if delta_y > 0 {
        // Going down, first move down on y axis
        for _ in 0..delta_y {
            moves.push(Move::Down);
        }

        // Move on x axis
        for _ in 0..delta_x.abs() {
            moves.push(move_x.clone());
        }
    } else {
        // Going up, first move on x axis
        for _ in 0..delta_x.abs() {
            moves.push(move_x.clone());
        }

        // Move up on x axis
        for _ in 0..delta_y.abs() {
            moves.push(Move::Up);
        }
    }

    moves
}

pub fn solve_second(_input: &str) -> usize {
    420
}

fn parse_input(input: &str) -> Vec<(Vec<char>, usize)> {
    let num_regex = Regex::new(r"^\d+").unwrap();
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            let chars = line.chars().collect();
            let num_value = num_regex.find(line).unwrap().as_str().parse().unwrap();
            (chars, num_value)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_works() {
        let input = "029A
        980A
        179A
        456A
        379A";

        assert_eq!(
            parse_input(input),
            vec![
                (vec!['0', '2', '9', 'A'], 29),
                (vec!['9', '8', '0', 'A'], 980),
                (vec!['1', '7', '9', 'A'], 179),
                (vec!['4', '5', '6', 'A'], 456),
                (vec!['3', '7', '9', 'A'], 379),
            ]
        );
    }

    #[test]
    fn numpad_moves_a_to_9() {
        assert_eq!(
            get_numpad_moves('A', '9'),
            MoveSearchResult::OneWay(vec![Move::Up, Move::Up, Move::Up])
        )
    }

    #[test]
    fn numpad_moves_1_to_3() {
        assert_eq!(
            get_numpad_moves('1', '3'),
            MoveSearchResult::OneWay(vec![Move::Right, Move::Right])
        )
    }

    #[test]
    fn numpad_moves_a_to_1() {
        assert_eq!(
            get_numpad_moves('A', '1'),
            MoveSearchResult::OneWay(vec![Move::Up, Move::Left, Move::Left])
        )
    }

    #[test]
    fn numpad_moves_7_to_0() {
        assert_eq!(
            get_numpad_moves('7', '0'),
            MoveSearchResult::OneWay(vec![Move::Right, Move::Down, Move::Down, Move::Down])
        )
    }

    #[test]
    fn numpad_moves_1_to_1() {
        assert_eq!(get_numpad_moves('1', '1'), MoveSearchResult::OneWay(vec![]))
    }

    #[test]
    fn arrowpad_moves_a_to_left() {
        assert_eq!(
            get_arrowpad_moves(Arrow::Activate, Arrow::Left),
            vec![Move::Down, Move::Left, Move::Left]
        )
    }

    #[test]
    fn arrowpad_moves_left_to_a() {
        assert_eq!(
            get_arrowpad_moves(Arrow::Left, Arrow::Activate),
            vec![Move::Right, Move::Right, Move::Up]
        )
    }

    #[test]
    fn arrowpad_moves_up_to_up() {
        assert_eq!(get_arrowpad_moves(Arrow::Up, Arrow::Up), vec![])
    }

    // #[test]
    // fn test_first_example_029a() {
    //     let input = "029A";

    //     assert_eq!(solve_first(input), 68 * 29);
    // }

    // #[test]
    // fn test_first_example_980a() {
    //     let input = "980A";

    //     assert_eq!(solve_first(input), 60 * 980);
    // }

    // #[test]
    // fn test_first_example_179a() {
    //     let input = "179A";

    //     assert_eq!(solve_first(input), 68 * 179);
    // }

    // #[test]
    // fn test_first_example_456a() {
    //     let input = "456A";

    //     assert_eq!(solve_first(input), 64 * 456);
    // }

    // #[test]
    // fn test_first_example_379a() {
    //     let input = "379A";

    //     assert_eq!(solve_first(input), 64 * 379);
    // }

    // #[test]
    // fn test_first_example_complete() {
    //     let input = "029A
    //         980A
    //         179A
    //         456A
    //         379A";

    //     assert_eq!(solve_first(input), 126384);
    // }

    // #[test]
    // fn test_second_example() {
    //     let input = "";
    //     assert_eq!(solve_second(input), 420);
    // }
}
