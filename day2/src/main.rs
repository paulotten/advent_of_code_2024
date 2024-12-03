mod input;

use input::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");

    let input = get_input();
    let reports = parse_input(input);

    let mut count_safe = 0;

    for report in reports {
        if report_is_safe(&report) {
            count_safe += 1;
        }
    }

    println!("{count_safe}")
}

fn report_is_safe(report: &Vec<i32>) -> bool {
    assert!(report.len() >= 2);

    let increasing = report[0] < report[1];

    report_is_safe_inner(&report, increasing, true)
}

fn part2() {
    println!("Part 2");

    let input = get_input();
    let reports = parse_input(input);

    let mut count_safe = 0;

    for report in reports {
        if report_is_safe2(&report) {
            count_safe += 1;
        }
    }

    println!("{count_safe}")
}

fn report_is_safe2(report: &Vec<i32>) -> bool {
    // skipping the first or second level could change if the report is increasing or decreasing
    // so try both
    report_is_safe_inner(&report, true, false) || report_is_safe_inner(&report, false, false)
}

fn report_is_safe_inner(report: &[i32], increasing: bool, skipped: bool) -> bool {
    for i in 1..report.len() {
        if !levels_are_safe(report[i - 1], report[i], increasing) {
            if skipped {
                return false;
            } else {
                // try skipping current level
                if i + 1 == report.len() {
                    return true;
                }

                if levels_are_safe(report[i - 1], report[i + 1], increasing)
                    && report_is_safe_inner(&report[i + 1..], increasing, true)
                {
                    return true;
                }

                // try skipping previous level
                if i == 1 || levels_are_safe(report[i - 2], report[i], increasing) {
                    return report_is_safe_inner(&report[i..], increasing, true);
                }

                return false;
            }
        }
    }

    true
}

fn levels_are_safe(a: i32, b: i32, increasing: bool) -> bool {
    let (smaller, larger) = if increasing { (a, b) } else { (b, a) };

    larger >= smaller + 1 && larger <= smaller + 3
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut reports = vec![];

    for line in input.lines() {
        let mut report = vec![];

        for part in line.split(" ") {
            let level = part.parse::<i32>().unwrap();

            report.push(level);
        }

        reports.push(report);
    }

    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_is_safe2_regression() {
        let report = vec![46, 47, 45, 48, 51, 52, 52];

        assert_eq!(report_is_safe2(&report), false);
    }
}
