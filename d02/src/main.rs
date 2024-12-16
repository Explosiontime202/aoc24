fn is_report_save(report: &Vec<u64>) -> bool {
    assert!(report.len() > 1);
    let is_ascending = report[0] < report[1];
    for i in 0..(report.len() - 1) {
        if (report[i] < report[i + 1]) != is_ascending {
            return false;
        }

        let diff = report[i].abs_diff(report[i + 1]);
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn solve_a(reports: &Vec<Vec<u64>>) -> u64 {
    let mut num_save_reports = 0;
    for report in reports {
        if is_report_save(report) {
            num_save_reports += 1;
        }
    }
    num_save_reports
}

fn solve_b(reports: &Vec<Vec<u64>>) -> u64 {
    let mut num_save_reports = 0;
    for report in reports {
        if is_report_save(report) {
            num_save_reports += 1;
            continue;
        }

        for i in 0..report.len() {
            let mut dampened_report = report.clone();
            dampened_report.remove(i);
            if is_report_save(&dampened_report) {
                num_save_reports += 1;
                break;
            }
        }
    }
    num_save_reports
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let reports: Vec<Vec<u64>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let output_a = solve_a(&reports);
    let output_b = solve_b(&reports);

    println!("Task1 = {output_a}");
    println!("Task2 = {output_b}");
}
