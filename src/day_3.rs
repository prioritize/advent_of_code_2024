use std::{
    fmt::write,
    fs::File,
    io::{BufRead, BufReader},
    path::Prefix,
};

pub fn parse_file(fname: &str) -> Vec<(u32, u32)> {
    let file = File::open(fname).unwrap();
    let mut output: Vec<(u32, u32)> = Vec::new();
    let lines = BufReader::new(file).lines();
    lines.for_each(|l| {
        let l = l.unwrap();
        let mul_split: Vec<&str> = l.split("mul(").collect();
        for entry in mul_split {
            let mut stage = entry.split(",");
            let left = match stage.next() {
                Some(v) => match v.parse::<u32>() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                },
                None => None,
            };
            let right = match stage.next() {
                Some(v) => {
                    let right = match v.split(")").next() {
                        Some(v) => match v.parse::<u32>() {
                            Ok(o) => Some(o),
                            Err(_) => None,
                        },
                        None => None,
                    };
                    right
                }
                None => None,
            };
            if left.is_some() && right.is_some() {
                output.push((left.unwrap(), right.unwrap()));
            }
        }
    });
    let product = output.iter().fold(0, |acc, (a, b)| acc + a * b);
    println!("Day 3 Part 1 Product: {}", product);
    output
}
pub fn parse_mul(input: &str) -> Option<u32> {
    let mut prefix = input.split("mul(");
    let junk = match prefix.next() {
        Some(v) => v,
        None => return None,
    };
    let mut comma_split = match prefix.next() {
        Some(v) => v,
        None => return None,
    }
    .split(",");
    let left = match comma_split.next() {
        Some(l) => match l.parse::<u32>() {
            Ok(l) => l,
            Err(_) => return None,
        },
        None => return None,
    };
    let right = match comma_split.next() {
        Some(v) => match v.split(")").next() {
            Some(v) => match v.parse::<u32>() {
                Ok(v) => v,
                Err(_) => return None,
            },
            None => return None,
        },
        None => return None,
    };
    Some(left * right)
}
pub fn day_3_parser(fname: &str, handle: bool) -> u32 {
    let file = File::open(fname).unwrap();
    let lines = BufReader::new(file).lines();
    let mut buffer: u32 = 0;
    let mut multiply_state = true;
    lines.for_each(|l| {
        let line = l.unwrap();
        line.chars()
            .enumerate()
            .for_each(|(idx, character)| match character {
                'm' => {
                    if multiply_state && advance_check("mul(", &line[idx..]) {
                        if let Some(v) = parse_mul(&line[idx..]) {
                            buffer += v;
                        }
                    }
                }
                'd' => {
                    if handle {
                        if advance_check("don't()", &line[idx..]) {
                            multiply_state = false;
                        } else if advance_check("do()", &line[idx..]) {
                            multiply_state = true;
                        }
                    }
                }
                _ => {}
            });
    });
    println!("total multiplied is: {buffer}");
    buffer
}
pub fn advance_check(incoming: &str, check_against: &str) -> bool {
    if incoming.len() < check_against.len() {
        *incoming == check_against[..incoming.len()]
    } else {
        false
    }
}
pub fn get_val_from_split(input: &str, split: &str) -> Option<u32> {
    let out = match input.split(split).next() {
        Some(v) => match v.parse::<u32>() {
            Ok(v) => Some(v),
            Err(_) => None,
        },
        None => None,
    };
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_3_parse_file() {
        let out = parse_file("input/day_3_input.txt");
    }
    #[test]
    fn test_advance_check_basic() {
        assert_eq!(true, advance_check("mul", "multitwentyone"));
    }
    #[test]
    fn test_advance_check_extra_after() {
        let test_val = "astmul(xyv";
        assert_eq!(true, advance_check("mul(", &test_val[3..]));
    }
    #[test]
    fn test_advance_check_method() {
        let output = day_3_parser("input/day_3_input.txt", false);
        assert_eq!(output, 170807108);
    }
    #[test]
    fn test_advance_check_method_true() {
        let output = day_3_parser("input/day_3_input.txt", true);
        assert_eq!(output, 7438033);
    }
}
