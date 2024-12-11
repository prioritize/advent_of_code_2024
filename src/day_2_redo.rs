use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[inline]
pub fn compare(a: &u32, b: &u32, delta: u32, func: fn(&u32, &u32, delta: u32) -> bool) -> bool {
    func(a, b, delta)
}
pub fn less(a: &u32, b: &u32, delta: u32) -> bool {
    let result = b - a;
    println!("{a} < {b} && {b} - {a} = {result} < {delta} && != 0");
    (a < b) && ((b - a) < delta) && (a != b)
}
pub fn more(a: &u32, b: &u32, delta: u32) -> bool {
    (a > b) && (a - b) < delta && (a != b)
}
pub fn eval(report: &[u32], func: fn(&u32, &u32, u32) -> bool) -> bool {
    println!("The report is: {:?}", report);
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
    println!("Called eval_dampen on {:?}", &report);
    for (idx, val) in report.iter().enumerate() {
        if idx + 1 < report.len() {
            if func(val, &report[idx + 1], 4) {
                return true;
            } else if idx + 2 < report.len() {
                println!("Check failed, but idx+2 is less than the length");
                let temp = &[&report[0..idx + 1], &report[(idx + 2)..]].concat();
                let location = idx + 2_usize;
                println!("Calling eval from eval_dampen: {} {:?}", val, &temp);
                let eval_out = eval(temp, func);
                println!("eval_out is {eval_out}");
            } else {
                println!("eval_dampen failed -- various reasons");
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
        reports
            .iter()
            .map(|report| {
                let out = eval_dampen(report, more) || eval_dampen(report, less);
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
