use crate::types::*;

pub fn solve(input: &str) -> String {
    let (mut registers, program) = parse_input(input);

    let mut instruction_pointer = 0;
    let mut output = Vec::<u64>::new();

    while instruction_pointer < program.len() {
        let opcode = Opcode::from(program[instruction_pointer]);
        let operand = program[instruction_pointer + 1];

        match opcode {
            Opcode::Adv => {
                registers.a =
                    Register(registers.a.0 >> ComboOperand::from(operand).value(&registers));
                instruction_pointer += 2;
            }
            Opcode::Bxl => {
                registers.b = Register(registers.b.0 ^ operand as u64);
                instruction_pointer += 2;
            }
            Opcode::Bst => {
                let value = ComboOperand::from(operand).value(&registers);
                registers.b = Register(value % 8);
                instruction_pointer += 2;
            }
            Opcode::Jnz => {
                if registers.a.0 == 0 {
                    instruction_pointer += 2;
                } else {
                    instruction_pointer = operand as usize;
                }
            }
            Opcode::Bxc => {
                registers.b = Register(registers.b.0 ^ registers.c.0);
                instruction_pointer += 2;
            }
            Opcode::Out => {
                let value = ComboOperand::from(operand).value(&registers) % 8;
                output.push(value);
                instruction_pointer += 2;
            }
            Opcode::Bdv => {
                registers.b =
                    Register(registers.a.0 >> ComboOperand::from(operand).value(&registers));
                instruction_pointer += 2;
            }
            Opcode::Cdv => {
                registers.c =
                    Register(registers.a.0 >> ComboOperand::from(operand).value(&registers));
                instruction_pointer += 2;
            }
        }
    }

    output
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn parse_input(input: &str) -> (Registers, Program) {
    let input = input.trim().split("\n\n").collect::<Vec<&str>>();

    let registers = input[0]
        .trim()
        .parse::<Registers>()
        .expect("Failed to parse registers");
    let program = input[1]
        .trim()
        .parse::<Program>()
        .expect("Failed to parse program");

    (registers, program)
}

#[cfg(test)]
mod test {
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
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }
}
