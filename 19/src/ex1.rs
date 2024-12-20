use crate::parser::{parse_input, Design};
use crate::trie::TrieNode;
use anyhow::Result;

pub fn solve(input: &str) -> Result<u32> {
    let (towels, designs) = parse_input(input)?;

    let mut trie = TrieNode::new();
    for towel in towels {
        trie.insert(towel.0);
    }

    let possible_design_count = designs
        .iter()
        .map(|design| is_design_possible(&trie, design) as u32)
        .sum();

    Ok(possible_design_count)
}

fn is_design_possible(trie: &TrieNode, design: &Design) -> bool {
    let n = design.0.len();
    let mut is_constructible = vec![false; n + 1];

    // can the substring design[0..i] be created using the towels
    is_constructible[0] = true;

    for i in 0..n {
        if !is_constructible[i] {
            continue;
        }

        let mut node = trie;
        let mut j = i;

        while j < n {
            let current_char = design
                .0
                .chars()
                .nth(j)
                .expect("{j} must be smaller than the length of {design}");

            if let Some(child) = node.children.get(&current_char) {
                node = child;
                j += 1;
                if node.is_end {
                    is_constructible[j] = true;
                }
            } else {
                break;
            }
        }
    }

    is_constructible[n]
}
