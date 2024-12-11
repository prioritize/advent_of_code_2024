use std::{
    fmt::write,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_file(fname: &str) -> Vec<(u32, u32)> {
    let file = File::open(fname).unwrap();
    let lines = BufReader::new(file).lines();
    lines
        .map(|l| {
            let l = l.unwrap();
            println!("{}", &l);
            let mul_split = l.split("mul(");

            let values: Vec<Option<(u32, u32)>> = mul_split
                .map(|v| {
                    let l_val = get_val_from_split(v, ",");
                    let r_val = get_val_from_split(v, ")");
                    if l_val.is_some() && r_val.is_some() {
                        Some((l_val.unwrap(), r_val.unwrap()))
                    } else {
                        None
                    }
                })
                .collect();
            values
        })
        .collect::<Vec<Vec<Option<(u32, u32)>>>>()
        .iter()
        .flatten()
        .flatten()
        .cloned()
        .collect()
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
