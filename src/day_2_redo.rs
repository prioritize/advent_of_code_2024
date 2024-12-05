fn less(a: &u32, b: &u32, delta: u32) -> bool {
    (a < b) && ((b - a) < delta) && !(a == b)
}
fn more(a: &u32, b: &u32, delta: u32) -> bool {
    (a > b) && (a - b) < delta && !(a == b)
}
fn eval(report: &[u32]) -> bool {
    let mut iter = report.iter().peekable();
    while let Some(v) = iter.next() {
        if let Some(n) = iter.peek() {
            if more(v, n, 3) || less(v, n, 3) {
                continue;
            } else {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        let test_val = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert!(eval(&test_val))
    }
    #[test]
    fn test_eval_fail() {
        let test_val = vec![1, 1, 3, 4, 5, 6, 7, 8];
        assert!(!eval(&test_val))
    }
    #[test]
    fn test_eval_fail_one() {
        let test_val = vec![1, 0, 3, 4, 5, 6, 7, 8];
        assert!(!eval(&test_val))
    }
    #[test]
    fn test_eval_fail_two() {
        let test_val = vec![1, 10, 3, 4, 5, 6, 7, 8];
        assert!(!eval(&test_val))
    }
}
