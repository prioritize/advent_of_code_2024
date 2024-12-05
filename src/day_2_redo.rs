fn less(a: &u32, b: &u32, delta: u32) -> bool {
    (a < b) && ((b - a) < delta) && !(a == b)
}
fn more(a: &u32, b: &u32, delta: u32) -> bool {}
fn eval(report: &[u32], cmp: fn(u32, u32, u32) -> bool) -> bool {
    let _ = report.iter().enumerate().map(|(idx, v)| {}).collect();
}
