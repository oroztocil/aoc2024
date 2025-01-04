#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;

pub fn solve_first(input: &str) -> String {
    let computer = parse_input(input);
    String::from("abraka")
}

pub fn solve_second(input: &str) -> String {
    String::from("dabra")
}

fn parse_input(input: &str) -> Computer {
    /*
    Register A: 729
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0
    */

    let values: Vec<&str> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(": ").unwrap())
        .map(|(_, second)| second)
        .collect();

    let register_a: usize = values[0].parse().unwrap();
    let register_b: usize = values[1].parse().unwrap();
    let register_c: usize = values[2].parse().unwrap();
    let instructions = values[3]
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        .chunks(2)
        .map(|pair| Instruction::parse(pair[0], pair[1]))
        .collect::<Vec<Instruction>>();

    Computer {
        instructions: Rc::from(instructions),
        state: State {
            register_a,
            register_b,
            register_c,
            ip: 0,
            output: Vec::<usize>::new(),
        },
    }
}

#[derive(Debug, PartialEq)]
struct Computer {
    instructions: Rc<[Instruction]>,
    state: State,
}

#[derive(Debug, PartialEq)]
struct State {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    ip: usize,
    output: Vec<usize>,
}

#[derive(Debug, PartialEq)]
enum Register {
    A,
    B,
    C,
}

type Literal = u8;

#[derive(Debug, PartialEq)]
enum ComboOperand {
    Literal(Literal),
    Register(Register),
}

impl ComboOperand {
    fn parse(operand: u8) -> Self {
        match operand {
            0..=3 => ComboOperand::Literal(operand),
            4 => ComboOperand::Register(Register::A),
            5 => ComboOperand::Register(Register::B),
            6 => ComboOperand::Register(Register::C),
            _ => panic!("Unsupported operand"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    /* adv 0 */ DivideToA(ComboOperand),
    /* bxl 1 */ XorBWithLiteral(Literal),
    /* bst 2 */ Modulo8ToB(ComboOperand),
    /* jnz 3 */ JumpIfANotZero(Literal),
    /* bxc 4 */ XorBWithC,
    /* out 5 */ Modulo8ToOut(ComboOperand),
    /* bdv 6 */ DivideToB(ComboOperand),
    /* cdv 7 */ DivideToC(ComboOperand),
}

impl Instruction {
    fn parse(opcode: u8, operand: u8) -> Self {
        match opcode {
            0 => Instruction::DivideToA(ComboOperand::parse(operand)),
            1 => Instruction::XorBWithLiteral(operand),
            2 => Instruction::Modulo8ToB(ComboOperand::parse(operand)),
            3 => Instruction::JumpIfANotZero(operand),
            4 => Instruction::XorBWithC,
            5 => Instruction::Modulo8ToOut(ComboOperand::parse(operand)),
            6 => Instruction::DivideToB(ComboOperand::parse(operand)),
            7 => Instruction::DivideToC(ComboOperand::parse(operand)),
            _ => panic!("Unsupported opcode"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input_file;

    use super::*;

    #[test]
    fn parse_input_works() {
        let computer = parse_input(&read_input_file("day17/test1.txt"));

        let expected = Computer {
            state: State {
                register_a: 729,
                register_b: 0,
                register_c: 0,
                ip: 0,
                output: vec![]
            },
            instructions: Rc::from(vec![
                Instruction::DivideToA(ComboOperand::Literal(1)),
                Instruction::Modulo8ToOut(ComboOperand::Register(Register::A)),
                Instruction::JumpIfANotZero(0)
            ])
        };

        assert_eq!(computer, expected);
    }

    // #[test]
    // fn test_first() {
    //     assert_eq!(
    //         solve_first(&read_input_file("day17/test1.txt")),
    //         "4,6,3,5,6,3,5,2,1,0"
    //     );
    // }

    // #[test]
    // fn test_second() {
    //     assert_eq!(solve_second(&read_input_file("day17/test1.txt")), "dabra");
    // }
}
