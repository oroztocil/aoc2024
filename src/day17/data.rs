use std::rc::Rc;

use super::operations::apply;

pub type Int = u64;

#[derive(Debug, PartialEq, Clone)]
pub struct Computer {
    pub instructions: Rc<[Instruction]>,
    pub state: State,
}

impl Computer {
    pub fn run_step(&mut self) -> Result<&State, ComputationHalted> {
        let instruction_index: usize = (self.state.registers.ip / 2) as usize;

        if instruction_index < self.instructions.len() {
            let instruction = &self.instructions[instruction_index];
            self.state = apply(self.state.clone(), instruction);
            Ok(&self.state)
        } else {
            Err(ComputationHalted(self.state.clone()))
        }
    }
}

pub struct ComputationHalted(pub State);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct RegisterState {
    pub a: Int,
    pub b: Int,
    pub c: Int,
    pub ip: Int,
}

#[derive(Debug, PartialEq, Clone)]
pub struct State {
    pub registers: RegisterState,
    pub output: Vec<Int>,
}

#[derive(Debug, PartialEq)]
pub enum Register {
    A,
    B,
    C,
}

#[derive(Debug, PartialEq)]
pub enum ComboOperand {
    Literal(Int),
    Register(Register),
}

impl ComboOperand {
    fn parse(operand: u8) -> Self {
        match operand {
            0..=3 => ComboOperand::Literal(operand.into()),
            4 => ComboOperand::Register(Register::A),
            5 => ComboOperand::Register(Register::B),
            6 => ComboOperand::Register(Register::C),
            _ => panic!("Unsupported operand"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// adv 0
    DivideToA(ComboOperand),

    /// bxl 1
    XorBWithLiteral(Int),

    /// bst 2
    Modulo8ToB(ComboOperand),

    /// jnz 3
    JumpIfANotZero(Int),

    /// bxc 4
    XorBWithC,

    /// out 5
    Modulo8ToOut(ComboOperand),

    /// bdv 6
    DivideToB(ComboOperand),

    /// cdv 7
    DivideToC(ComboOperand),
}

impl Instruction {
    pub fn parse(opcode: u8, operand: u8) -> Self {
        match opcode {
            0 => Instruction::DivideToA(ComboOperand::parse(operand)),
            1 => Instruction::XorBWithLiteral(operand.into()),
            2 => Instruction::Modulo8ToB(ComboOperand::parse(operand)),
            3 => Instruction::JumpIfANotZero(operand.into()),
            4 => Instruction::XorBWithC,
            5 => Instruction::Modulo8ToOut(ComboOperand::parse(operand)),
            6 => Instruction::DivideToB(ComboOperand::parse(operand)),
            7 => Instruction::DivideToC(ComboOperand::parse(operand)),
            _ => panic!("Unsupported opcode"),
        }
    }
}

pub fn parse_input(input: &str) -> (Computer, Vec<Int>) {
    let values: Vec<&str> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(": ").unwrap())
        .map(|(_, second)| second)
        .collect();

    let register_a: Int = values[0].parse().unwrap();
    let register_b: Int = values[1].parse().unwrap();
    let register_c: Int = values[2].parse().unwrap();
    let codes = values[3]
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    let instructions = codes
        .chunks(2)
        .map(|pair| Instruction::parse(pair[0], pair[1]))
        .collect::<Vec<Instruction>>();

    let computer = Computer {
        instructions: Rc::from(instructions),
        state: State {
            registers: RegisterState {
                a: register_a,
                b: register_b,
                c: register_c,
                ip: 0,
            },
            output: Vec::<Int>::new(),
        },
    };

    let codes = codes.iter().map(|c| *c as Int).collect();

    (computer, codes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test_input_is_correct() {
        let input = "Register A: 729
            Register B: 0
            Register C: 0

            Program: 0,1,5,4,3,0";

        let expected = Computer {
            state: State {
                registers: RegisterState {
                    a: 729,
                    b: 0,
                    c: 0,
                    ip: 0,
                },
                output: vec![],
            },
            instructions: Rc::from(vec![
                Instruction::DivideToA(ComboOperand::Literal(1)),
                Instruction::Modulo8ToOut(ComboOperand::Register(Register::A)),
                Instruction::JumpIfANotZero(0),
            ]),
        };

        let (result, _) = parse_input(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_problem_input_is_correct() {
        let input = "Register A: 28066687
            Register B: 0
            Register C: 0

            Program: 2,4,1,1,7,5,4,6,0,3,1,4,5,5,3,0";

        let expected = Computer {
            state: State {
                registers: RegisterState {
                    a: 28066687,
                    b: 0,
                    c: 0,
                    ip: 0,
                },
                output: vec![],
            },
            instructions: Rc::from(vec![
                Instruction::Modulo8ToB(ComboOperand::Register(Register::A)),
                Instruction::XorBWithLiteral(1),
                Instruction::DivideToC(ComboOperand::Register(Register::B)),
                Instruction::XorBWithC,
                Instruction::DivideToA(ComboOperand::Literal(3)),
                Instruction::XorBWithLiteral(4),
                Instruction::Modulo8ToOut(ComboOperand::Register(Register::B)),
                Instruction::JumpIfANotZero(0),
            ]),
        };

        let (result, _) = parse_input(input);

        assert_eq!(result, expected);
    }
}
