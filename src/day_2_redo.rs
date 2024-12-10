use color_eyre::Report;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[inline]
pub fn compare(a: &u32, b: &u32, delta: u32, func: fn(&u32, &u32, delta: u32) -> bool) -> bool {
    func(a, b, delta)
}
pub fn less(a: &u32, b: &u32, delta: u32) -> bool {
    (a < b) && ((b - a) < delta) && (a != b)
}
pub fn more(a: &u32, b: &u32, delta: u32) -> bool {
    (a > b) && (a - b) < delta && (a != b)
}
pub fn eval(report: &[u32], func: fn(&u32, &u32, u32) -> bool) -> bool {
    let mut iter = report.iter().peekable();
    while let Some(v) = iter.next() {
        if let Some(n) = iter.peek() {
            if func(v, n, 4) {
                continue;
            } else {
                return false;
            }
        }
    }
    true
}
pub fn delta_difference(report: &[u32]) -> Vec<(i32, i32)> {
    report
        .iter()
        .enumerate()
        .map(|(idx, val)| match idx < report.len() - 1 {
            true => {
                let neighbor = report[idx + 1] as i32;
                let val = *val as i32;
                Some(((val - neighbor), (neighbor - val)))
            }
            false => None,
        })
        .collect::<Vec<Option<(i32, i32)>>>()
        .into_iter()
        .flatten()
        .collect()
}
pub fn eval_delta(deltas: Vec<(i32, i32)>, max_delta: i32, dampen: bool) -> u32 {
    let checks: Vec<bool> = deltas
        .iter()
        .enumerate()
        .map(|(idx, (lesser, greater))| {
            (!lesser.is_negative() && *lesser < max_delta)
                || (!greater.is_negative() && *greater < max_delta)
        })
        .collect();
    println!("{:?}", checks);
    let test_val = checks
        .into_iter()
        .filter(|x| *x)
        .collect::<Vec<bool>>()
        .iter()
        .len() as u32;
    println!("{}", test_val);
    todo!()
}

pub fn eval_dampen(report: &[u32], func: fn(&u32, &u32, u32) -> bool) -> bool {
    println!("incoming data: {:?}", report);
    for (idx, val) in report.iter().enumerate() {
        if idx + 1 < report.len() {
            if func(val, &report[idx + 1], 4) {
                continue;
            } else if idx + 2 < report.len() {
                let temp = &[&report[idx..idx + 1], &report[(idx + 2)..]].concat();
                println!("Temp report is: {:?}", temp);
                return eval(temp, func);
            } else {
                if eval(&report[1..], func) {
                    return true;
                }
                if eval(&report[..=(report.len() - 2)], func) {
                    return true;
                }
                return false;
            }
        }
    }
    true
}

pub fn check_all_reports(reports: Vec<Vec<u32>>, dampen: bool) -> u32 {
    if !dampen {
        reports
            .iter()
            .map(|report| {
                let out = eval(report, more) || eval(report, less);
                if out {
                } else {
                    println!("{:?} -- UNSAFE", report);
                }
                out
            })
            .collect::<Vec<bool>>()
            .iter()
            .map(|r| if *r { 1 } else { 0 })
            .collect::<Vec<u32>>()
            .iter()
            .sum()
    } else {
        reports
            .iter()
            .map(|report| {
                let out = eval_dampen(report, more) || eval_dampen(report, less);
                println!("results of eval_dampen: {}", out);
                if out {
                } else {
                    println!("{:?} -- UNSAFE", report);
                }
                out
            })
            .collect::<Vec<bool>>()
            .iter()
            .map(|r| if *r { 1 } else { 0 })
            .collect::<Vec<u32>>()
            .iter()
            .sum()
    }
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

pub fn check_all_reports_delta(reports: Vec<Vec<u32>>) -> u32 {
    let deltas: Vec<Vec<(i32, i32)>> = reports.iter().map(|r| delta_difference(r)).collect();
    deltas.iter().map(|d| eval_delta(d, 4, false))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        let test_val = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert!(eval(&test_val, less))
    }
    #[test]
    fn test_eval_fail() {
        let test_val = vec![1, 1, 3, 4, 5, 6, 7, 8];
        assert!(!eval(&test_val, more))
    }
    #[test]
    fn test_eval_fail_one() {
        let test_val = vec![1, 0, 3, 4, 5, 6, 7, 8];
        assert!(!eval(&test_val, more))
    }
    #[test]
    fn test_eval_fail_two() {
        let test_val = vec![1, 10, 3, 4, 5, 6, 7, 8];
        assert!(!eval(&test_val, more))
    }
    #[test]
    fn test_example_input() {
        let reports = read_file_to_line("input/day_2_test_input.txt");
        assert!(2 == check_all_reports(reports, false));
    }
    #[test]
    fn test_input() {
        let reports = read_file_to_line("input/day_2_input.txt");
        assert_eq!(564, check_all_reports(reports, false));
    }
    #[test]
    fn test_test_input_dampened() {
        let reports = read_file_to_line("input/day_2_test_input.txt");
        assert_eq!(4, check_all_reports(reports, true));
    }
    #[test]
    fn test_input_dampened() {
        let reports = read_file_to_line("input/day_2_input.txt");
        assert_eq!(564, check_all_reports(reports, true));
    }
}
