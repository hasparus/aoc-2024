use std::collections::VecDeque;

use crate::parse_input::{parse_input, Gate, GateKind};

pub fn solve(input: &str) -> u64 {
    let (mut wires, gates) = parse_input(input);

    let mut queue = VecDeque::from(gates);

    while let Some(gate) = queue.pop_front() {
        let in1 = wires.get(&gate.input.left);
        let in2 = wires.get(&gate.input.right);

        if let Some(in1) = in1 {
            if let Some(in2) = in2 {
                let value = match gate.input.kind {
                    GateKind::And => in1 & in2,
                    GateKind::Or => in1 | in2,
                    GateKind::Xor => in1 ^ in2,
                };

                wires.insert(gate.output, value);
                continue;
            }
        }

        queue.push_back(gate);
    }

    let z_byte = "z".as_bytes()[0];

    let res = wires.iter().fold(0, |acc, (name, value)| {
        if name.0[0] == z_byte {
            let position = ((name.0[1] - b'0') * 10 + (name.0[2] - b'0')) as u64;
            acc | ((*value as u64) << position)
        } else {
            acc
        }
    });

    res
}
