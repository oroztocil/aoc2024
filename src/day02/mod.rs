use crate::utils::drop_element;

pub fn solve_first(input: &str) -> usize {
    let reports = get_reports(input);
    reports.iter().filter(|r| is_report_valid(r, 0)).count()
}

pub fn solve_second(input: &str) -> usize {
    let reports = get_reports(input);
    reports.iter().filter(|r| is_report_valid(r, 1)).count()
}

fn is_report_valid(report: &Vec<i32>, max_errors: i32) -> bool {
    check_report(report, is_valid_increase, max_errors)
        || check_report(report, is_valid_decrease, max_errors)
}

fn is_valid_increase(a: i32, b: i32) -> bool {
    let diff = b - a;
    diff > 0 && diff <= 3
}

fn is_valid_decrease(a: i32, b: i32) -> bool {
    let diff = b - a;
    diff < 0 && diff >= -3
}

fn check_report(report: &Vec<i32>, condition: fn(i32, i32) -> bool, errors_left: i32) -> bool {
    let first_error = (0..report.len() - 1).find(|i| !condition(report[*i], report[*i + 1]));

    match (first_error, errors_left) {
        (None, _) => true,
        (Some(_), 0) => false,
        (Some(i), n) => {
            let new_report = drop_element(report, i);
            let new_report2 = drop_element(report, i + 1);
            check_report(&new_report, condition, n - 1)
                || check_report(&new_report2, condition, n - 1)
        }
    }
}

fn get_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|c| c.parse::<i32>().unwrap()).collect())
        .collect()
}
