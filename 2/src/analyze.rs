use aoc_2024_2::{ex_2, ex_2_brute, parse_input};
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let parsed_input = parse_input(&input);

    // Compare results for each report
    for (i, report) in parsed_input.iter().enumerate() {
        let brute_result = ex_2_brute::count_safe_reports(&vec![report.clone()]);
        let ex2_result = ex_2::count_safe_reports_with_problem_dampener(&vec![report.clone()]);

        if brute_result != ex2_result {
            println!("Discrepancy found in report #{}", i + 1);
            println!("Report: {:?}", report);
            println!("Brute force result: {}", brute_result);
            println!("Ex2 result: {}", ex2_result);

            // If brute force says it's valid, show which element can be removed to make it valid
            if brute_result == 1 {
                for (j, _) in report.iter().enumerate() {
                    let derived = [report[..j].to_vec(), report[j + 1..].to_vec()].concat();
                    let is_increasing = derived[1] > derived[0];
                    let is_valid = derived.windows(2).all(|window| {
                        let delta = window[1] - window[0];
                        (1..=3).contains(&delta.abs())
                            && (is_increasing && delta > 0 || !is_increasing && delta < 0)
                    });
                    if is_valid {
                        println!(
                            "Removing element at index {} (value {}) makes it valid",
                            j, report[j]
                        );
                        println!("Resulting sequence: {:?}", derived);
                        break;
                    }
                }
            }
            println!("---");
        }
    }

    // Print total counts
    let brute_total = ex_2_brute::count_safe_reports(&parsed_input);
    let ex2_total = ex_2::count_safe_reports_with_problem_dampener(&parsed_input);
    println!("Total reports: {}", parsed_input.len());
    println!("Brute force total: {}", brute_total);
    println!("Ex2 total: {}", ex2_total);
}
