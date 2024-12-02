use super::*;

pub fn count_safe_reports(input: &ParsedInput) -> u32 {
    input
        .iter()
        .filter(|report| {
            let is_increasing = report[1] > report[0];
            report.windows(2).all(|window| {
                let delta = window[1] - window[0];
                (1..=3).contains(&delta.abs())
                    && (is_increasing && delta > 0 || !is_increasing && delta < 0)
            })
        })
        .count() as u32
} 
