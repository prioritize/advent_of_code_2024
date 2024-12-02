use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn day_1_part_1(fname: &str) {
    let fname = File::open(fname).unwrap();
    let buf = BufReader::new(fname);
    let mut stuff = buf.lines();
    let mut first_buffer = Vec::new();
    let mut second_buffer = Vec::new();
    stuff.for_each(|line| {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let first = split.next().unwrap().parse::<u32>().unwrap();
        let second = split.next().unwrap().parse::<u32>().unwrap();
        first_buffer.push(first);
        second_buffer.push(second);
    });
    first_buffer.sort();
    second_buffer.sort();
    let mut distance_buffer = Vec::new();
    for i in 0..first_buffer.len() {
        if first_buffer[i] > second_buffer[i] {
            distance_buffer.push(first_buffer[i] - second_buffer[i]);
        } else if first_buffer[i] < second_buffer[i] {
            distance_buffer.push(second_buffer[i] - first_buffer[i]);
        } else {
            distance_buffer.push(0);
        }
    }
    println!(
        "The sum of all the distances is {}",
        distance_buffer.iter().sum::<u32>()
    );
}
pub fn day_1_part_2(fname: &str) {
    let fname = File::open(fname).unwrap();
    let buf = BufReader::new(fname);
    let mut stuff = buf.lines();
    let mut first_buffer = Vec::new();
    let mut second_buffer = Vec::new();
    stuff.for_each(|line| {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let first = split.next().unwrap().parse::<u32>().unwrap();
        let second = split.next().unwrap().parse::<u32>().unwrap();
        first_buffer.push(first);
        second_buffer.push(second);
    });
    first_buffer.sort();
    second_buffer.sort();
    let mut entries: HashMap<u32, u32> = HashMap::new();
    for item in second_buffer {
        *entries.entry(item).or_insert(0) += 1;
    }
    let output = first_buffer
        .into_iter()
        .map(|k| {
            return match entries.get(&k) {
                Some(v) => k * v,
                None => 0,
            };
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();
    println!("The output for part 2 is {}", output);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_1_part_1() {
        day_1_part_1("input/day_1_input.txt");
    }
    #[test]
    fn test_day_1_part_2() {
        day_1_part_2("input/day_1_input.txt");
    }
}
