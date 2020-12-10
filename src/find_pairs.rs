use std::ops::Add;

pub trait PairFinder<T>
where
    T: Add<Output = T> + Eq,
{
    fn find_pair_with_sum(&self, sum: T) -> Option<(T, T)>;
}

impl<T> PairFinder<T> for [T]
where
    T: Add<Output = T> + Ord + Eq + Copy,
{
    fn find_pair_with_sum(&self, sum: T) -> Option<(T, T)> {
        for (i, &a) in self.iter().enumerate() {
            if a >= sum {
                continue;
            }
            for &b in self.iter().skip(i + 1) {
                if a + b == sum {
                    return Some((a, b));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_day_one() {
        let nums = vec![1721, 979, 366, 299, 675, 1456];
        // the Vec itself
        assert_eq!(Some((1721, 299)), nums.find_pair_with_sum(2020));
        // as a slice
        assert_eq!(Some((1721, 299)), nums[..5].find_pair_with_sum(2020));
        assert_eq!(None, nums[1..].find_pair_with_sum(2020));
    }
}
