fn ex_1_distances(input: &str) -> Result<u32, String> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut nums = line.split_whitespace();
        let Some(l) = nums.next() else {
            continue;
        };
        let Some(r) = nums.next() else {
            return Err(format!("No right number in {}", line));
        };

        left.push(l.parse::<u32>().unwrap());
        right.push(r.parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum())
}

fn ex_2_similarity_score(input: &str) -> Result<u32, String> {
    let mut counts = std::collections::HashMap::<u32, u32>::new();

    let mut left = Vec::<u32>::new();

    for line in input.lines() {
        let mut nums = line.split_whitespace();
        let Some(l) = nums.next().and_then(|s| s.parse::<u32>().ok()) else {
            continue;
        };
        let Some(r) = nums.next().and_then(|s| s.parse::<u32>().ok()) else {
            return Err(format!("No right number in {}", line));
        };

        left.push(l);
        *counts.entry(r).or_insert(0) += 1;
    }

    Ok(left
        .iter()
        .map(|l| l * counts.get(l).unwrap_or(&0))
        .sum::<u32>())
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    println!("{:?}", ex_1_distances(&input).unwrap());
    println!("{:?}", ex_2_similarity_score(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE: &str = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "#;

    #[test]
    fn distances_on_example() {
        assert_eq!(ex_1_distances(SIMPLE), Ok(11));
    }

    #[test]
    fn similarity_score_on_example() {
        assert_eq!(ex_2_similarity_score(SIMPLE), Ok(31));
    }
}
