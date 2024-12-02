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
        let (smaller, larger) = if increasing {
            (report[i - 1], report[i])
        } else {
            (report[i], report[i - 1])
        };

        if larger < smaller + 1 || larger > smaller + 3 {
            if skipped {
                return false;
            } else {
                // try removing current level
                let mut copy = report.to_vec();
                copy.remove(i);

                if report_is_safe_inner(&copy, increasing, true) {
                    return true;
                }

                // try removing previous level
                let mut copy = report.to_vec();
                copy.remove(i - 1);

                return report_is_safe_inner(&copy, increasing, true);
            }
        }
    }

    true
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
