struct Problem {
    target: u64,
    nums: Vec<u64>,
}

struct SubProblem<'a> {
    target: u64,
    sub_result: u64,
    nums: &'a Vec<u64>,
    nums_index: usize,
}

impl<'a> SubProblem<'a> {
    fn from_problem(problem: &'a Problem) -> Self {
        SubProblem {
            target: problem.target,
            sub_result: 0,
            nums: &problem.nums,
            nums_index: 0,
        }
    }

    fn apply_operation(&self, operation: &Operation) -> ComputationState<'a> {
        if self.nums_index == self.nums.len() {
            if self.sub_result == self.target {
                ComputationState::Succesful
            } else {
                ComputationState::Failed
            }
        } else {
            let new_result = match operation {
                Operation::Add => self.sub_result + self.nums[self.nums_index],
                Operation::Multiply => self.sub_result * self.nums[self.nums_index],
                Operation::Concat => format!("{}{}", self.sub_result, self.nums[self.nums_index])
                    .parse::<u64>()
                    .unwrap(),
            };

            if new_result > self.target {
                ComputationState::Failed
            } else {
                ComputationState::Incomplete(SubProblem {
                    target: self.target,
                    sub_result: new_result,
                    nums: self.nums,
                    nums_index: self.nums_index + 1,
                })
            }
        }
    }
}

enum Operation {
    Add,
    Multiply,
    Concat,
}

enum ComputationState<'a> {
    Failed,
    Succesful,
    Incomplete(SubProblem<'a>),
}

pub fn solve_first(input: &str) -> usize {
    solve(input, &[Operation::Multiply, Operation::Add])
}

pub fn solve_second(input: &str) -> usize {
    solve(input, &[Operation::Concat, Operation::Multiply, Operation::Add])
}

fn solve(input: &str, operations: &[Operation]) -> usize {
    parse_input(input)
        .iter()
        .map(SubProblem::from_problem)
        .filter(|sub_problem| is_feasible(sub_problem, &operations))
        .map(|problem| problem.target)
        .sum::<u64>() as usize
}

fn is_feasible(sub_problem: &SubProblem, operations: &[Operation]) -> bool {
    operations
        .iter()
        .any(|operation| match sub_problem.apply_operation(operation) {
            ComputationState::Failed => false,
            ComputationState::Succesful => true,
            ComputationState::Incomplete(next) => is_feasible(&next, operations),
        })
}

fn parse_input(input: &str) -> Vec<Problem> {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(first, second)| Problem {
            target: first.parse::<u64>().unwrap(),
            nums: second
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input_file;

    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first(&read_input_file("day07/test1.txt")), 3749);
    }

    #[test]
    fn test_first_b() {
        assert_eq!(solve_first(&read_input_file("day07/test2.txt")), 9);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second(&read_input_file("day07/test1.txt")), 11387);
    }
}
