use super::*;

pub fn count_safe_reports(input: &ParsedInput) -> u32 {
    input
        .iter()
        .filter(|report| {
            // all reports derived by removing element at index i
            let mut derived_reports = report
                .iter()
                .enumerate()
                .map(|(i, _)| [report[..i].to_vec(), report[i + 1..].to_vec()].concat());

            let is_safe = |report: &[i32]| {
                let is_increasing = report[1] > report[0];
                report.windows(2).all(|window| {
                    let delta = window[1] - window[0];
                    (1..=3).contains(&delta.abs())
                        && (is_increasing && delta > 0 || !is_increasing && delta < 0)
                })
            };

            derived_reports.any(|derived_report| is_safe(&derived_report))
        })
        .count() as u32
} 
