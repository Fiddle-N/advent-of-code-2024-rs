use itertools::Itertools;

advent_of_code::solution!(2);

fn all_positive(report_diffs: &Vec<i32>) -> bool {
    report_diffs.iter().all(|&x| x > 0)
}

fn all_negative(report_diffs: &Vec<i32>) -> bool {
    report_diffs.iter().all(|&x| x < 0)
}

fn within_range(report_diffs: &Vec<i32>) -> bool {
    report_diffs.iter().all(|&x| x.abs() >= 1 && x.abs() <= 3)
}

fn is_safe(report_diffs: &Vec<i32>) -> bool {
    (all_positive(&report_diffs) || all_negative(&report_diffs)) && within_range(&report_diffs)
}

fn report_diffs(report: &Vec<i32>) -> Vec<i32> {
    let mut report_diffs = vec![];
    for (a, b) in report.iter().tuple_windows() {
        report_diffs.push(a - b)
    }
    report_diffs
}

fn shameless_copies(report: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut shameless_copies = vec![];
    for (i, _) in report.iter().enumerate() {
        let mut diff_copy = report.clone();
        diff_copy.remove(i);
        shameless_copies.push(diff_copy);
    }
    shameless_copies
}

fn safe_reports(input: &str, dampener: bool) -> u32 {
    let reports = input.lines().map(|line| {
        line.split_whitespace()
            .map(|level| level.parse::<i32>().expect("All levels fit into i32"))
            .collect::<Vec<_>>()
    });
    let safety = reports.map(|report| {
        let report_diff = report_diffs(&report);
        let safe_report = is_safe(&report_diff);
        if dampener && !safe_report {
            let report_copies = shameless_copies(&report);
            report_copies
                .iter()
                .any(|report| is_safe(&report_diffs(&report)))
        } else {
            safe_report
        }
    });
    let result: i32 = safety.map(|x| x as i32).sum();
    let result: u32 = result.try_into().expect("result will be positive");
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(safe_reports(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(safe_reports(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
