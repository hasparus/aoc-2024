use crate::parse_input::{parse_input, Gate, GateKind, Wire};
use std::collections::HashSet;

fn is_direct(gate: &Gate) -> bool {
    gate.input.left.0[0] == b'x' || gate.input.right.0[0] == b'x'
}

fn is_output(gate: &Gate) -> bool {
    gate.output.0[0] == b'z'
}

fn has_output(output: &Wire) -> impl Fn(&Gate) -> bool + '_ {
    move |gate| gate.output == *output
}

fn has_input(input: &Wire) -> impl Fn(&Gate) -> bool + '_ {
    move |gate| gate.input.left == *input || gate.input.right == *input
}

pub fn solve(input: &str) -> String {
    let (wires_map, gates) = parse_input(input);
    let input_bit_count = wires_map.len() / 2;

    let mut flags = HashSet::new();

    let fa_gate0s: Vec<_> = gates
        .iter()
        .filter(|g| is_direct(g) && g.input.kind == GateKind::Xor)
        .collect();

    for gate in &fa_gate0s {
        let is_first = gate.input.left.0[0] == b'x'
            && gate.input.left.0[1] == b'0'
            && gate.input.left.0[2] == b'0'
            || gate.input.right.0[0] == b'x'
                && gate.input.right.0[1] == b'0'
                && gate.input.right.0[2] == b'0';

        if is_first {
            if gate.output.0 != [b'z', b'0', b'0'] {
                flags.insert(gate.output);
            }
            continue;
        } else if gate.output.0 == [b'z', b'0', b'0'] {
            flags.insert(gate.output);
        }

        if is_output(gate) {
            flags.insert(gate.output);
        }
    }

    let fa_gate3s: Vec<_> = gates
        .iter()
        .filter(|g| g.input.kind == GateKind::Xor && !is_direct(g))
        .collect();

    for gate in &fa_gate3s {
        if !is_output(gate) {
            flags.insert(gate.output);
        }
    }

    let output_gates: Vec<_> = gates.iter().filter(|g| is_output(g)).collect();
    for gate in &output_gates {
        let last_output = format!("z{:02}", input_bit_count);
        let is_last = gate.output.to_string() == last_output;

        if is_last {
            if gate.input.kind != GateKind::Or {
                flags.insert(gate.output);
            }
            continue;
        } else if gate.input.kind != GateKind::Xor {
            flags.insert(gate.output);
        }
    }

    let mut check_next = Vec::new();
    for gate in &fa_gate0s {
        if flags.contains(&gate.output) || gate.output.0 == [b'z', b'0', b'0'] {
            continue;
        }

        let matches: Vec<_> = fa_gate3s
            .iter()
            .filter(|g| has_input(&gate.output)(g))
            .collect();
        if matches.is_empty() {
            check_next.push(gate);
            flags.insert(gate.output);
        }
    }

    for gate in check_next {
        let a_slice = &gate.input.left.0[1..];
        let intended_result = Wire([b'z', a_slice[0], a_slice[1]]);

        let matches: Vec<_> = fa_gate3s
            .iter()
            .filter(|g| has_output(&intended_result)(g))
            .collect();

        if matches.len() != 1 {
            panic!("Critical Error! Is your input correct?");
        }

        let match_gate = matches[0];
        let to_check = [match_gate.input.left, match_gate.input.right];

        let or_matches: Vec<_> = gates
            .iter()
            .filter(|g| g.input.kind == GateKind::Or && to_check.contains(&g.output))
            .collect();

        if or_matches.len() != 1 {
            panic!("Critical Error! This solver isn't complex enough");
        }

        let or_match_output = or_matches[0].output;
        let correct_output = to_check
            .iter()
            .find(|&&output| output != or_match_output)
            .expect("Should find correct output");

        flags.insert(*correct_output);
    }

    let mut result: Vec<_> = flags.iter().map(|w| w.to_string()).collect();
    result.sort();
    result.join(",")
}
