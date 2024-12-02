use super::ParsedInput;

fn is_safe(report: &[i32], problem_dampener_quota: i32) -> bool {
    if report.len() < 2 {
        return false;
    }

    fn is_delta_ok(is_increasing: bool, delta: i32) -> bool {
        (is_increasing && delta > 0 || !is_increasing && delta < 0)
            && (1..=3).contains(&delta.abs())
    }

    // First check if sequence is valid without removing anything
    let is_increasing = report[1] > report[0];
    let mut all_valid = true;
    for window in report.windows(2) {
        let delta = window[1] - window[0];
        if !is_delta_ok(is_increasing, delta) {
            all_valid = false;
            break;
        }
    }
    if all_valid {
        return true;
    }

    if problem_dampener_quota > 0 && report.len() >= 3 {
        // Try removing each number
        for skip_idx in 0..report.len() {
            let mut valid = true;
            let mut prev = None;
            let mut next = None;
            let mut is_increasing = None;

            // First, find a valid pair to determine direction
            for i in 0..report.len() {
                if i == skip_idx {
                    continue;
                }
                let curr = report[i];

                if prev.is_none() {
                    prev = Some(curr);
                    continue;
                }
                if next.is_none() {
                    next = Some(curr);
                    let delta = curr - prev.unwrap();
                    if (1..=3).contains(&delta.abs()) {
                        is_increasing = Some(delta > 0);
                        break;
                    }
                    prev = Some(curr);
                    next = None;
                }
            }

            // If we found a valid direction, check the rest of the sequence
            if let Some(increasing) = is_increasing {
                let mut prev = None;
                for i in 0..report.len() {
                    if i == skip_idx {
                        continue;
                    }
                    let curr = report[i];

                    if let Some(p) = prev {
                        let delta = curr - p;
                        if !is_delta_ok(increasing, delta) {
                            valid = false;
                            break;
                        }
                    }
                    prev = Some(curr);
                }

                if valid {
                    return true;
                }
            }
        }
    }

    false
}

pub fn count_safe_reports_with_problem_dampener(input: &ParsedInput) -> u32 {
    input.iter().filter(|report| is_safe(report, 1)).count() as u32
}
