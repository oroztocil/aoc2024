use data::{parse_input, ComputationHalted, Computer, Int, RegisterState, State};

mod data;
mod operations;

pub fn solve_first(input: &str) -> String {
    let (computer, _) = parse_input(input);
    let final_state = run_until_halt(computer);
    serialize_output(&final_state)
}

pub fn solve_second(input: &str) -> Int {
    let (original_computer, complete_output) = parse_input(input);

    let mut candidates: Vec<Int> = vec![0];

    for inst_count in 1..=complete_output.len() {
        let target_output = complete_output[(complete_output.len() - inst_count)..].to_vec();
        let mut next_candidates: Vec<Int> = Vec::new();

        for c in candidates {
            let next_base = c * 8;

            for n in next_base..next_base + 8 {
                next_candidates.push(n);
            }
        }

        candidates = Vec::new();

        for candidate_a in next_candidates {
            let computer = Computer {
                instructions: original_computer.instructions.clone(),
                state: State {
                    output: vec![],
                    registers: RegisterState {
                        a: candidate_a,
                        b: 0,
                        c: 0,
                        ip: 0,
                    },
                },
            };

            if run_matches_output(computer, &target_output) {
                candidates.push(candidate_a);
            }
        }
    }

    candidates.into_iter().min().unwrap()
}

fn run_until_halt(mut computer: Computer) -> State {
    loop {
        match computer.run_step() {
            Ok(_) => {}
            Err(ComputationHalted(final_state)) => return final_state,
        }
    }
}

fn run_matches_output(mut computer: Computer, target_output: &Vec<Int>) -> bool {
    let mut output_len = 0;

    loop {
        match computer.run_step() {
            Ok(_) => {}
            Err(ComputationHalted(_)) => break,
        }

        if computer.state.output.len() > output_len {
            if computer.state.output[output_len] != target_output[output_len] {
                break;
            }

            output_len = computer.state.output.len();

            if output_len >= target_output.len() {
                break;
            }
        }
    }

    computer.state.output == *target_output
}

fn serialize_output(state: &State) -> String {
    state
        .output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input_file;

    use super::*;

    #[test]
    fn test_first_example() {
        assert_eq!(
            solve_first(&read_input_file("day17/test1.txt")),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }

    #[test]
    fn test_first_example_2() {
        let input = "Register A: 10
            Register B: 0
            Register C: 0

            Program: 5,0,5,1,5,4";

        let expected_output = "0,1,2";
        let result = solve_first(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_first_example_3() {
        let input = "Register A: 2024
            Register B: 0
            Register C: 0

            Program: 0,1,5,4,3,0";

        let expected_output = "4,2,5,6,7,7,7,7,3,1,0";

        let (computer, _) = parse_input(input);
        let final_state = run_until_halt(computer);
        let result = serialize_output(&final_state);

        assert_eq!(final_state.registers.a, 0);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_first_example_4() {
        let input = "Register A: 117440
            Register B: 0
            Register C: 0

            Program: 0,3,5,4,3,0";

        let expected_output = "0,3,5,4,3,0";
        let result = solve_first(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_first_solution() {
        let input = "Register A: 28066687
            Register B: 0
            Register C: 0

            Program: 2,4,1,1,7,5,4,6,0,3,1,4,5,5,3,0";

        let expected_output = "7,3,0,5,7,1,4,0,5";
        let result = solve_first(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_cooking() {
        let input = "Register A: 46
            Register B: 0
            Register C: 0

            Program: 2,4,1,1,7,5,4,6,0,3,1,4,5,5,3,0";

        let expected_output = "3,0";
        let result = solve_first(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_cooking_2() {
        let input = "Register A: 368
            Register B: 0
            Register C: 0

            Program: 2,4,1,1,7,5,4,6,0,3,1,4,5,5,3,0";

        let expected_output = "5,3,0";
        let result = solve_first(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_second_example() {
        assert_eq!(solve_second(&read_input_file("day17/test2.txt")), 117440);
    }

    #[test]
    fn radek_input() {
        let input = "Register A: 25986278
            Register B: 0
            Register C: 0

            Program: 2,4,1,4,7,5,4,1,1,4,5,5,0,3,3,0";

        assert_eq!(solve_second(input), 156985331222018);
    }

    #[test]
    fn tomik_input() {
        let input = "Register A: 66245665
            Register B: 0
            Register C: 0

            Program: 2,4,1,7,7,5,1,7,4,6,0,3,5,5,3,0";

        assert_eq!(solve_second(input), 265061364597659);
    }
}
