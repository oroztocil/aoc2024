use super::data::{ComboOperand, Instruction, Int, Register, RegisterState, State};

pub fn apply(state: State, instruction: &Instruction) -> State {
    match instruction {
        Instruction::DivideToA(operand) => {
            /*
               The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
            */
            let result = compute_div_operation(&state, resolve_combo_operand(&state, operand));

            State {
                registers: RegisterState {
                    a: result,
                    ip: state.registers.ip + 2,
                    ..state.registers
                },
                ..state
            }
        }
        Instruction::DivideToB(operand) => {
            /*
               The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
            */
            let result = compute_div_operation(&state, resolve_combo_operand(&state, operand));

            State {
                registers: RegisterState {
                    b: result,
                    ip: state.registers.ip + 2,
                    ..state.registers
                },
                ..state
            }
        }
        Instruction::DivideToC(operand) => {
            /*
               The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
            */
            let result = compute_div_operation(&state, resolve_combo_operand(&state, operand));

            State {
                registers: RegisterState {
                    c: result,
                    ip: state.registers.ip + 2,
                    ..state.registers
                },
                ..state
            }
        }
        Instruction::XorBWithLiteral(value) => {
            /*
               The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
            */
            let result = compute_xor_operation(&state, *value);

            State {
                registers: RegisterState {
                    b: result,
                    ip: state.registers.ip + 2,
                    ..state.registers
                },
                ..state
            }
        }
        Instruction::XorBWithC => {
            /*
               The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
            */
            let result = compute_xor_operation(&state, state.registers.c);

            State {
                registers: RegisterState {
                    b: result,
                    ip: state.registers.ip + 2,
                    ..state.registers
                },
                ..state
            }
        }
        Instruction::Modulo8ToB(operand) => {
            /*
               The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
            */
            let result = compute_mod_operation(resolve_combo_operand(&state, operand));

            State {
                registers: RegisterState {
                    b: result,
                    ip: state.registers.ip + 2,
                    ..state.registers
                },
                ..state
            }
        }
        Instruction::Modulo8ToOut(operand) => {
            /*
               The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
            */
            let result = compute_mod_operation(resolve_combo_operand(&state, operand));
            let mut new_output = state.output.clone();
            new_output.push(result);

            State {
                output: new_output,
                registers: RegisterState {
                    ip: state.registers.ip + 2,
                    ..state.registers
                },
            }
        }
        Instruction::JumpIfANotZero(value) => {
            /*
               The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
            */

            if state.registers.a == 0 {
                State {
                    registers: RegisterState {
                        ip: state.registers.ip + 2,
                        ..state.registers
                    },
                    ..state
                }
            } else {
                State {
                    registers: RegisterState {
                        ip: *value,
                        ..state.registers
                    },
                    ..state
                }
            }
        }
    }
}

fn compute_div_operation(state: &State, operand_value: Int) -> Int {
    let numerator = state.registers.a;
    let denominator = (2 as Int).pow(operand_value.try_into().unwrap());
    numerator / denominator
}

fn compute_xor_operation(state: &State, operand_value: Int) -> Int {
    state.registers.b ^ operand_value
}

fn compute_mod_operation(operand: Int) -> Int {
    operand % 8
}

fn resolve_combo_operand(state: &State, operand: &ComboOperand) -> Int {
    match operand {
        ComboOperand::Literal(value) => *value as Int,
        ComboOperand::Register(register) => match register {
            Register::A => state.registers.a,
            Register::B => state.registers.b,
            Register::C => state.registers.c,
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::day17::data::{ComboOperand, RegisterState, State};

    use super::*;

    #[test]
    fn apply_divide_to_a_is_correct() {
        let initial_state = State {
            registers: RegisterState {
                a: 729,
                b: 0,
                c: 0,
                ip: 0,
            },
            output: vec![],
        };

        let expected_state = State {
            registers: RegisterState {
                a: 91,
                b: 0,
                c: 0,
                ip: 2,
            },
            output: vec![],
        };

        let instruction = Instruction::DivideToA(ComboOperand::Literal(3));
        let result_state = apply(initial_state, &instruction);

        assert_eq!(result_state, expected_state);
    }

    #[test]
    fn apply_divide_to_b_is_correct() {
        let initial_state = State {
            registers: RegisterState {
                a: 729,
                b: 0,
                c: 0,
                ip: 0,
            },
            output: vec![],
        };

        let expected_state = State {
            registers: RegisterState {
                a: 729,
                b: 91,
                c: 0,
                ip: 2,
            },
            output: vec![],
        };

        let instruction = Instruction::DivideToB(ComboOperand::Literal(3));
        let result_state = apply(initial_state, &instruction);

        assert_eq!(result_state, expected_state);
    }

    #[test]
    fn apply_divide_to_c_is_correct() {
        let initial_state = State {
            registers: RegisterState {
                a: 729,
                b: 0,
                c: 0,
                ip: 0,
            },
            output: vec![],
        };

        let expected_state = State {
            registers: RegisterState {
                a: 729,
                b: 0,
                c: 91,
                ip: 2,
            },
            output: vec![],
        };

        let instruction = Instruction::DivideToC(ComboOperand::Literal(3));
        let result_state = apply(initial_state, &instruction);

        assert_eq!(result_state, expected_state);
    }

    #[test]
    fn apply_xor_b_with_literal_is_corect() {
        let initial_state = State {
            registers: RegisterState {
                a: 0,
                b: 29,
                c: 0,
                ip: 0,
            },
            output: vec![],
        };

        let expected_state = State {
            registers: RegisterState {
                a: 0,
                b: 26,
                c: 0,
                ip: 2,
            },
            output: vec![],
        };

        let instruction = Instruction::XorBWithLiteral(7);
        let result_state = apply(initial_state, &instruction);

        assert_eq!(result_state, expected_state);
    }

    #[test]
    fn apply_xor_b_with_c_is_corect() {
        let initial_state = State {
            registers: RegisterState {
                a: 0,
                b: 2024,
                c: 43690,
                ip: 0,
            },
            output: vec![],
        };

        let expected_state = State {
            registers: RegisterState {
                a: 0,
                b: 44354,
                c: 43690,
                ip: 2,
            },
            output: vec![],
        };

        let instruction = Instruction::XorBWithC;
        let result_state = apply(initial_state, &instruction);

        assert_eq!(result_state, expected_state);
    }

    #[test]
    fn apply_modulo_8_to_b_is_corect() {
        let initial_state = State {
            registers: RegisterState {
                a: 0,
                b: 0,
                c: 9,
                ip: 0,
            },
            output: vec![],
        };

        let expected_state = State {
            registers: RegisterState {
                a: 0,
                b: 1,
                c: 9,
                ip: 2,
            },
            output: vec![],
        };

        let instruction = Instruction::Modulo8ToB(ComboOperand::Register(Register::C));
        let result_state = apply(initial_state, &instruction);

        assert_eq!(result_state, expected_state);
    }

    #[test]
    fn apply_modulo_8_to_out_is_corect() {
        let initial_state = State {
            registers: RegisterState {
                a: 0,
                b: 0,
                c: 9,
                ip: 0,
            },
            output: vec![],
        };

        let expected_state = State {
            registers: RegisterState {
                a: 0,
                b: 0,
                c: 9,
                ip: 2,
            },
            output: vec![1],
        };

        let instruction = Instruction::Modulo8ToOut(ComboOperand::Register(Register::C));
        let result_state = apply(initial_state, &instruction);

        assert_eq!(result_state, expected_state);
    }

    #[test]
    fn apply_jump_if_a_not_zero_when_a_is_0_is_corect() {
        let initial_state = State {
            registers: RegisterState {
                a: 0,
                b: 0,
                c: 0,
                ip: 0,
            },
            output: vec![],
        };

        let expected_state = State {
            registers: RegisterState {
                a: 0,
                b: 0,
                c: 0,
                ip: 2,
            },
            output: vec![],
        };

        let instruction = Instruction::JumpIfANotZero(42);
        let result_state = apply(initial_state, &instruction);

        assert_eq!(result_state, expected_state);
    }

    #[test]
    fn apply_jump_if_a_not_zero_when_a_is_not_0_is_corect() {
        let initial_state = State {
            registers: RegisterState {
                a: 42,
                b: 0,
                c: 0,
                ip: 0,
            },
            output: vec![],
        };

        let expected_state = State {
            registers: RegisterState {
                a: 42,
                b: 0,
                c: 0,
                ip: 10,
            },
            output: vec![],
        };

        let instruction = Instruction::JumpIfANotZero(10);
        let result_state = apply(initial_state, &instruction);

        assert_eq!(result_state, expected_state);
    }
}
