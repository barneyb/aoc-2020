pub fn find_pair_with_sum(expenses: &[i64], sum: i64) -> Option<(i64, i64)> {
    for (j, &b) in expenses.iter().enumerate() {
        let remaining = sum - b;
        if remaining <= 0 {
            continue;
        }
        for &c in expenses.iter().skip(j + 1) {
            if c == remaining {
                return Some((b, c));
            }
        }
    }
    None
}
