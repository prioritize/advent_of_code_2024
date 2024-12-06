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
pub fn eval_dampen(report: &[u32], func: fn(&u32, &u32, u32) -> bool) -> bool {
    let mut skipped = false;
    for (idx, val) in report.iter().enumerate() {}
    true
}

pub fn check_all_reports(reports: Vec<Vec<u32>>, dampen: bool) -> u32 {
    if dampen {
        reports
            .iter()
            .map(|report| {
                let out = eval(report, more) || eval(report, less);
                if out {
                    println!("{:?} -- SAFE", report);
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
        0
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
}
