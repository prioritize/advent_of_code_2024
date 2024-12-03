#![allow(clippy::single_match)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn day_2_part_1(fname: &str) -> u32 {
    let reports = read_file_to_line(fname);
    println!("Length of reports: {}", reports.len());
    let evals = reports
        .iter()
        .map(|r| eval_report_safety(r))
        .collect::<Vec<bool>>();
    let safe: Vec<&bool> = evals.iter().filter(|e| e == &&true).collect();
    println!("{:?}", safe);
    safe.len() as u32
}

pub fn read_file_to_line(fname: &str) -> Vec<Vec<u32>> {
    let file = File::open(fname).unwrap_or_else(|_| panic!("Unable to open {}", fname));
    let lines = BufReader::new(file).lines();
    lines
        .map(|l| {
            let mut temp_vec = Vec::new();
            let line = l.unwrap();
            let split = line.split_whitespace();
            for value in split {
                temp_vec.push(value.parse::<u32>().unwrap());
            }
            temp_vec
        })
        .collect()
}
pub fn eval_report_safety_part_2(report: &[u32]) -> bool {
    if (check_monotonic_decreasing(report) || check_monotonic_increasing(report))
        && check_max_delta(report)
    {
        return true;
    }
    false
}
pub fn eval_report_safety(report: &[u32]) -> bool {
    if (check_monotonic_decreasing(report) || check_monotonic_increasing(report))
        && check_max_delta(report)
    {
        return true;
    }
    false
}
pub fn check_monotonic_decreasing(report: &[u32]) -> bool {
    for idx in 0..report.len() - 1 {
        if report[idx + 1] < report[idx] {
            continue;
        } else {
            return false;
        }
    }
    true
}
pub fn evaluate_reports(report: &u32, allow_single: bool) -> bool {
    todo!()
}
pub fn calc_differences(report: &[u32]) -> Vec<u32> {
    let mut differences = Vec::new();
    for i in 0..report.len() - 1 {
        let diff = report[i] as i32 - report[i + 1] as i32;
        differences.push(diff);
    }
    todo!()
}

pub fn check_monotonic_increasing(report: &[u32]) -> bool {
    let mut has_jumped = false;
    for idx in 0..report.len() - 1 {
        if report[idx + 1] > report[idx] {
            continue;
        } else {
            return false;
        }
    }
    true
}
pub fn check_max_delta(report: &[u32]) -> bool {
    if report[0] > report[1] {
        // Decreasing
        for i in 0..report.len() - 1 {
            match report[i] > report[i + 1] {
                true => match report[i] - report[i + 1] < 4 {
                    true => {}
                    false => return false,
                },
                false => return false,
            }
        }
        return true;
    }
    if report[0] < report[1] {
        // Increasing
        for i in 0..report.len() - 1 {
            match report[i + 1] > report[i] {
                true => match report[i + 1] - report[i] < 4 {
                    true => {}
                    false => return false,
                },
                false => return false,
            }
        }
        return true;
    }
    if report[0] == report[1] {
        return false;
    }
    // for idx in 0..report.len() - 1 {
    //     let delta = report[idx + 1] as i32 - report[idx] as i32;
    //
    //     match delta {
    //         0 => return false,
    //         1..=3 => return true,
    //         -3..=-1 => return true,
    //         _ => return false,
    //     }
    // }
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_2_part_1() {
        let safe_reports = day_2_part_1("input/day_2_input.txt");
        println!("The number of safe reports is {}", safe_reports);
    }
    #[test]
    fn test_day_2_part_1_test_input() {
        let safe_reports = day_2_part_1("input/day_2_test_input.txt");
        println!("The number of safe test reports is {}", safe_reports);
    }
    #[test]
    fn test_monotonic_decreasing() {
        let input: Vec<u32> = vec![7, 6, 4, 2, 1];
        assert!(check_monotonic_decreasing(&input));
    }
    #[test]
    fn test_monotonic_increasing_pass() {
        let input: Vec<u32> = vec![1, 2, 7, 8, 9];
        assert!(check_monotonic_increasing(&input));
    }
    #[test]
    fn test_monotonic_decreasing_and_max_delta() {
        let input: Vec<u32> = vec![1, 2, 7, 8, 9];
        assert!(!(check_monotonic_decreasing(&input) && check_max_delta(&input)));
    }
    #[test]
    fn test_monotonic_increasing_and_max_delta() {
        let input: Vec<u32> = vec![1, 2, 7, 8, 9];
        assert!(!(check_monotonic_increasing(&input) && check_max_delta(&input)));
    }
    #[test]
    fn test_monotonic_increasing_fail() {
        let input: Vec<u32> = vec![7, 6, 4, 2, 1];
        assert!(!check_monotonic_increasing(&input));
    }
}
