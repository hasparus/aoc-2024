use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum SymbolicValue {
    Known(u64),
    ShiftRight(Box<SymbolicValue>, Box<SymbolicValue>),
    Xor(Box<SymbolicValue>, Box<SymbolicValue>),
    Modulo(Box<SymbolicValue>, u64),
    A,
    B,
    C,
    Literal(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SymbolicRegisters {
    a: SymbolicValue,
    b: SymbolicValue,
    c: SymbolicValue,
}

#[derive(Debug, Clone)]
struct ExecutionState {
    registers: SymbolicRegisters,
    instruction_pointer: usize,
    output: Vec<SymbolicValue>,
}

#[derive(Debug)]
enum Execution {
    Terminal(ExecutionState),
    Branch {
        condition: SymbolicValue,
        if_true: Box<Execution>,
        if_false: Box<Execution>,
    },
}

impl ExecutionState {
    fn new() -> Self {
        Self {
            registers: SymbolicRegisters {
                a: SymbolicValue::A,
                b: SymbolicValue::Known(0),
                c: SymbolicValue::Known(0),
            },
            instruction_pointer: 0,
            output: Vec::new(),
        }
    }

    fn step(&self, program: &[u8]) -> Option<Execution> {
        if self.instruction_pointer >= program.len() {
            return Some(Execution::Terminal(self.clone()));
        }

        let opcode = Opcode::from(program[self.instruction_pointer]);
        let operand = program[self.instruction_pointer + 1];

        let mut next_state = self.clone();
        next_state.instruction_pointer += 2;

        match opcode {
            Opcode::Adv => {
                next_state.registers.a = SymbolicValue::ShiftRight(
                    Box::new(self.registers.a.clone()),
                    Box::new(self.get_combo_value(operand)),
                );
                Some(Execution::Terminal(next_state))
            }
            Opcode::Bxl => {
                next_state.registers.b = SymbolicValue::Xor(
                    Box::new(self.registers.b.clone()),
                    Box::new(SymbolicValue::Known(operand as u64)),
                );
                Some(Execution::Terminal(next_state))
            }
            Opcode::Bst => {
                next_state.registers.b =
                    SymbolicValue::Modulo(Box::new(self.get_combo_value(operand)), 8);
                Some(Execution::Terminal(next_state))
            }
            Opcode::Jnz => {
                let mut jump_state = self.clone();
                jump_state.instruction_pointer = operand as usize;

                Some(Execution::Branch {
                    condition: self.registers.a.clone(),
                    if_true: Box::new(jump_state.step(program)?),
                    if_false: Box::new(next_state.step(program)?),
                })
            }
            Opcode::Bxc => {
                next_state.registers.b = SymbolicValue::Xor(
                    Box::new(self.registers.b.clone()),
                    Box::new(self.registers.c.clone()),
                );
                Some(Execution::Terminal(next_state))
            }
            Opcode::Out => {
                next_state.output.push(SymbolicValue::Modulo(
                    Box::new(self.get_combo_value(operand)),
                    8,
                ));
                Some(Execution::Terminal(next_state))
            }
            Opcode::Bdv => {
                next_state.registers.b = SymbolicValue::ShiftRight(
                    Box::new(self.registers.a.clone()),
                    Box::new(self.get_combo_value(operand)),
                );
                Some(Execution::Terminal(next_state))
            }
            Opcode::Cdv => {
                next_state.registers.c = SymbolicValue::ShiftRight(
                    Box::new(self.registers.a.clone()),
                    Box::new(self.get_combo_value(operand)),
                );
                Some(Execution::Terminal(next_state))
            }
        }
    }

    fn get_combo_value(&self, operand: u8) -> SymbolicValue {
        match ComboOperand::from(operand) {
            ComboOperand::Zero => SymbolicValue::Literal(0),
            ComboOperand::One => SymbolicValue::Literal(1),
            ComboOperand::Two => SymbolicValue::Literal(2),
            ComboOperand::Three => SymbolicValue::Literal(3),
            ComboOperand::A => self.registers.a.clone(),
            ComboOperand::B => self.registers.b.clone(),
            ComboOperand::C => self.registers.c.clone(),
            ComboOperand::Reserved => panic!("Reserved combo operand"),
        }
    }
}

pub fn solve(input: &str) -> String {
    let (_, program) = crate::ex1::parse_input(input);
    let initial_state = ExecutionState::new();
    let execution = initial_state
        .step(&program.0)
        .expect("Failed to execute program");

    println!("{:?}", execution);

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2024_lib::input_reader::read_input;

    fn get_input(name: &str) -> String {
        let inputs = read_input("./inputs.md").expect("Failed to read input");
        let input = inputs.get_input(name);
        input.content.clone()
    }

    #[test]
    fn test_example() {
        let example = get_input("Example");
        let result = solve(&example);
        assert_eq!(result, "117440");
    }
}
