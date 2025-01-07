use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Problem {
    answer: u64,
    numbers: Vec<u64>,
    operations: &'static [char],
}
impl Problem {
    pub fn new(answer: u64, numbers: Vec<u64>) -> Self {
        Problem { answer, numbers }
    }
    pub fn evaluate(&self) -> {

    }
}
fn parse_file(fname: &str) -> Vec<Problem> {
    let file = File::open(fname).unwrap();
    let lines = BufReader::new(file).lines();
    lines
        .into_iter()
        .map(|l| {
            let l = l.unwrap();
            let mut first_split = l.split(":");
            let answer = first_split.next().unwrap().parse::<u64>().unwrap();
            let numbers = first_split
                .next()
                .unwrap()
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            Problem::new(answer, numbers)
        })
        .collect()
}
fn permutations(set: &[char], prefix: &str, n: u32, k: u32, buffer: &mut Vec<String>) {
    if k == 0 {
        buffer.push(prefix.to_string());
    } else {
        for idx in 0..n {
            let mut new_prefix = String::new();
            new_prefix = prefix.to_owned() + &set[idx as usize].to_string();
            // println!("{new_prefix}");
            permutations(set, &new_prefix, n, k - 1, buffer);
        }
    }
}
pub fn generate_all_permutations() -> Vec<String> {
    let set = ['+', 'x'];
    let prefix = String::new();
    let mut buffer = Vec::new();
    permutations(&set, &prefix, 2, 4, &mut buffer);
    buffer
}

pub fn day_7_part_1(fname: &str) -> u32 {
    let problems = parse_file(fname);
    let max_length = problems.iter().map(|x| x.numbers.len()).collect::<&u32>()
    let set = ['+', 'x'];
    problems
        .iter()
        .map(|x| 7)
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_day_7() {
        let permutations = generate_all_permutations();

    }
}
