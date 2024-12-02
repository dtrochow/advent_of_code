use aoc_lib::read_lines;

type Report = Vec<i32>;

fn get_reports(lines: Vec<String>) -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();

    for line in lines {
        reports.push(
            line.split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect(),
        );
    }

    reports
}

fn is_diff_within_limits(report: &Report) -> bool {
    let mut prev_value = report.first().unwrap();
    for value in &report[1..] {
        if (value - prev_value).abs() > 3 {
            return false;
        }
        prev_value = value;
    }
    true
}

fn is_all_increasing(report: &Report) -> bool {
    let mut prev_value = report.first().unwrap();
    for value in &report[1..] {
        if value <= prev_value {
            return false;
        }
        prev_value = value;
    }
    true
}

fn is_all_decreasing(report: &Report) -> bool {
    let mut prev_value = report.first().unwrap();
    for value in &report[1..] {
        if value >= prev_value {
            return false;
        }
        prev_value = value;
    }
    true
}

fn is_report_safe(report: Report) -> bool {
    is_diff_within_limits(&report) && (is_all_increasing(&report) || is_all_decreasing(&report))
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let reports = get_reports(lines);

    let mut safe_count = 0;
    for report in reports {
        if is_report_safe(report) {
            safe_count += 1;
        }
    }

    println!("Safe reports count: {}", safe_count);
}
