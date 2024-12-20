use crate::parser::{parse_input, Design};
use crate::trie::TrieNode;
use anyhow::Result;

pub fn solve(input: &str) -> Result<u64> {
    let (towels, designs) = parse_input(input)?;

    let mut trie = TrieNode::new();
    for towel in towels {
        trie.insert(towel.0);
    }

    let total_combinations = designs
        .iter()
        .map(|design| count_design_combinations(&trie, design))
        .sum();

    Ok(total_combinations)
}

fn count_design_combinations(trie: &TrieNode, design: &Design) -> u64 {
    let n = design.0.len();
    let mut combinations = vec![0u64; n + 1];

    // empty design can be constructed in just one way – using no towels
    combinations[0] = 1;

    for i in 0..n {
        if combinations[i] == 0 {
            continue;
        }

        let mut node = trie;
        let mut j = i;

        while j < n {
            let current_char = design.0.chars().nth(j).expect("valid index");

            if let Some(child) = node.children.get(&current_char) {
                node = child;
                j += 1;

                if node.is_end {
                    combinations[j] += combinations[i];
                }
            } else {
                break;
            }
        }
    }

    combinations[n]
}
