use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct PrintQueue {
    orders: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}
impl PrintQueue {
    fn check_updates(&self) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
        let mut out_collection: Vec<bool> = vec![];
        let mut correct_lists: Vec<Vec<u32>> = vec![];
        let mut incorrect_lists: Vec<Vec<u32>> = vec![];
        for print_order in &self.updates {
            match self.check_potential_manual(print_order) {
                true => correct_lists.push(print_order.clone()),
                false => incorrect_lists.push(print_order.clone()),
            }
        }
        let middle_sum: u32 = correct_lists
            .iter()
            .map(|x| {
                let middle = x.len() / 2;
                x[middle]
            })
            .collect::<Vec<u32>>()
            .iter()
            .sum();

        println!("middle sum: {}", middle_sum);
        println!("length of correct_lists:{:?}", correct_lists.len());
        println!("length of incorrect_lists:{:?}", incorrect_lists.len());

        println!("length of lines: {}", self.updates.len());
        (correct_lists, incorrect_lists)
    }
    fn check_potential_manual(&self, manual: &[u32]) -> bool {
        for (idx, entry) in manual.iter().enumerate() {
            let temp = &manual[idx + 1..];
            for page in temp {
                if let Some(p) = self.orders.get(page) {
                    if p.contains(entry) {
                        return false;
                    }
                }
            }
        }
        true
    }
    fn reorder_incorrect_manual(&self, manual: &[u32]) -> Vec<u32> {
        let mut swap_storage: Vec<&u32> = vec![];
        let mut buffer: Vec<&u32> = vec![];
        for (idx, entry) in manual.iter().enumerate() {
            let temp = &manual[idx + 1..];
            for page in temp {
                if let Some(p) = self.orders.get(page) {
                    if p.contains(entry) {
                        swap_storage.push(page);
                    }
                }
            }
            match swap_storage.len() {
                0 => {
                    if !buffer.contains(&entry) {
                        buffer.push(entry)
                    }
                }
                _ => {
                    for swap in &swap_storage {
                        if !buffer.contains(swap) {
                            buffer.push(swap);
                        }
                    }
                    swap_storage.clear();
                    buffer.push(entry);
                }
            }
        }
        let temp_buffer: Vec<u32> = buffer.into_iter().copied().collect();
        match self.check_potential_manual(&temp_buffer) {
            true => temp_buffer,
            false => self.reorder_incorrect_manual(&temp_buffer),
        }
    }
    pub fn reorder_all_incorrect(&self, incorrect: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
        incorrect
            .iter()
            .map(|x| {
                let in_len = x.len();
                let reordered = self.reorder_incorrect_manual(x);
                let out_len = reordered.len();
                if in_len != out_len {
                    println!("incoming: {:?}", x);
                    println!("outgoing: {:?}", reordered);
                    println!("{} -- {}", in_len, out_len);
                }

                reordered
            })
            .collect()
    }
}
pub fn sum_of_middle(manuals: Vec<Vec<u32>>) -> u32 {
    manuals
        .iter()
        .map(|x| x[x.len() / 2])
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}
pub fn parse_input(fname: &str) -> PrintQueue {
    let mut orders: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<u32> = vec![];
    let mut order_lines = vec![];
    let mut update_lines = vec![];
    let file = File::open(fname).unwrap();
    let lines = BufReader::new(file).lines();
    lines.for_each(|l| {
        let line = l.unwrap();
        if line.contains("|") {
            order_lines.push(line.clone())
        }
        if line.contains(",") {
            update_lines.push(line.clone());
        }
    });
    let pq = PrintQueue {
        orders: parse_orders(order_lines),
        updates: parse_updates(update_lines),
    };
    pq
}
fn parse_orders(incoming: Vec<String>) -> HashMap<u32, Vec<u32>> {
    let mut out: HashMap<u32, Vec<u32>> = HashMap::new();
    incoming.iter().for_each(|l| {
        let mut split = l.split("|");
        let a = split.next().unwrap().parse::<u32>().unwrap();
        let b = split.next().unwrap().parse::<u32>().unwrap();
        out.entry(a).and_modify(|e| e.push(b)).or_insert(vec![b]);
    });
    out
}
fn parse_updates(incoming: Vec<String>) -> Vec<Vec<u32>> {
    incoming
        .iter()
        .map(|l| {
            let split = l.split(",");
            split
                .map(|e| e.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_5_parse_input() {
        parse_input("input/day_5_input.txt");
    }
    #[test]
    fn test_day_5_parse_test_input() {
        let pq = parse_input("input/day_5_test_input.txt");
        let (correct, incorrect) = pq.check_updates();
        pq.reorder_all_incorrect(incorrect);
    }
    #[test]
    fn test_day_5_parse_test_reorder() {
        let pq = parse_input("input/day_5_input.txt");
        let (correct, incorrect) = pq.check_updates();
        let reordered = pq.reorder_all_incorrect(incorrect);
        let middle_sum = sum_of_middle(reordered);
        println!("Output: {middle_sum}");
    }
}
