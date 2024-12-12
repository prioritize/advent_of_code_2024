use std::{
    fmt::write,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_file(fname: &str) -> Vec<(u32, u32)> {
    let file = File::open(fname).unwrap();
    let lines = BufReader::new(file).lines();
    let mut l: Vec<Vec<Option<(u32, u32)>>> = lines
        .map(|l| {
            let l = l.unwrap();
            let out = l
                .split_inclusive("mul(")
                .map(|x| {
                    let mut l_split = x.split(",");
                    let out = match l_split.next() {
                        Some(l) => match l.parse::<u32>() {
                            Ok(left) => {
                                let left = Some(left);
                                let right = match l_split.next() {
                                    Some(v) => match v.split(")").next() {
                                        Some(v) => match v.parse::<u32>() {
                                            Ok(v) => Some(v),
                                            Err(_) => None,
                                        },
                                        None => None,
                                    },
                                    None => None,
                                };
                                if left.is_some() && right.is_some() {
                                    Some((left.unwrap(), right.unwrap()))
                                } else {
                                    None
                                }
                            }
                            Err(_) => None,
                        },
                        None => todo!(),
                    };
                    out
                })
                .collect::<Vec<Option<(u32, u32)>>>();
            out
        })
        .collect();
    let temp = l
        .iter()
        .map(|outer| {
            outer
                .into_iter()
                .filter(|inner| inner.is_some())
                .collect::<Vec<Option<(u32, u32)>>>()
                .iter()
                .flatten()
                .cloned()
                .collect()
        })
        .collect();
    temp
}

pub fn get_val_from_split(input: &str, split: &str) -> Option<u32> {
    println!("{input}");
    let out = match input.split(split).next() {
        Some(v) => match v.parse::<u32>() {
            Ok(v) => Some(v),
            Err(_) => None,
        },
        None => None,
    };
    println!("{:?}", out);
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_3_parse_file() {
        let out = parse_file("input/day_3_input.txt");
        println!("{:?}", out);
    }
}
