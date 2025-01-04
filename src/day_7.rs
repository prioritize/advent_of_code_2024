use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Problem {
    answer: u64,
    numbers: Vec<u64>,
}
impl Problem {
    pub fn new(answer: u64, numbers: Vec<u64>) -> Self {
        Problem { answer, numbers }
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
fn permutations(set: &[char], prefix: &str, n: u32, k: u32) {
    if k == 0 {
        println!("{prefix}");
        return;
    }
    for idx in 0..n {
        let mut new_prefix = String::new();
        new_prefix = prefix.to_owned() + &set[idx as usize].to_string();
        // println!("{new_prefix}");
        permutations(set, &new_prefix, n, k - 1)
    }
}
fn print_all_permutations() {
    let set = ['+', 'x'];
    let prefix = String::new();
    permutations(&set, &prefix, 2, 4)
}
fn day_7_part_1

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_day_7() {
        print_all_permutations();
        // parse_file("input/day_7_input.txt");
    }
}
