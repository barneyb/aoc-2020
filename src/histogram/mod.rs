use std::collections::HashMap;
use std::hash::Hash;

/// Histograms are a set of buckets with a count of how many items are in the bucket. The buckets
/// are usually a set of discrete segments which cover a continuous interval, but don't have to be.
/// Buckets always start with a count of zero.
///
/// The two primary implementations are:
///
/// 1.  `Vec<usize>` which requires `usize` bucket labels, and is fast for small numbers of buckets
///     that have labels close to zero.
/// 1.  `HashMap<T, usize>` which allows arbitrary `Eq + Hash` bucket labels, and is better suited
///     to sparsely filled buckets or those which don't naturally map onto `usize`.
///
/// # Examples
///
/// Ages of people at a kid's birthday party, using `Vec` w/ `usize` as the bucket type:
///
/// ```
/// use aoc_2020::histogram::Histogram;
///
/// let ages = vec![
///     0, 1, 2, 3, // younger siblings
///     5, 5, 5, 5, // friends
///     6,          // HAPPY BIRTHDAY TO YOU!
///     6, 6, 6, 6, // more friends
///     34, 35,     // parents
///     62, 62, 63  // grandparents
/// ];
/// let mut hist = Vec::new();
/// for a in ages {
///     hist.increment(a);
/// }
/// assert_eq!(4, hist.get_count(&5));
/// assert_eq!(0, hist.get_count(&7));
/// assert_eq!(0, hist.get_count(&99));
/// assert_eq!(Some(&4), hist.get(5));
/// assert_eq!(Some(&0), hist.get(7));
/// assert_eq!(None, hist.get(99));
/// ```
///
/// Cups of coffee by day of week, using as `HashMap` w/ `String` as the bucket type.
///
/// ```
/// use aoc_2020::histogram::Histogram;
/// use std::collections::HashMap;
///
/// let cups = vec!["mon", "mon", "tue", "wed", "wed", "wed", "fri"];
/// let mut hist = HashMap::new();
/// for c in cups {
///     hist.increment(c);
/// }
/// assert_eq!(3, hist.get_count(&"wed"));
/// assert_eq!(0, hist.get_count(&"sat"));
/// assert_eq!(Some(&3), hist.get(&"wed"));
/// assert_eq!(None, hist.get(&"sat"));
/// ```
///
pub trait Histogram<T> {
    /// Increment a bucket by one and return the new value.
    fn increment(&mut self, bucket: T) -> usize {
        self.increment_by(bucket, 1)
    }

    /// Increment a bucket by an arbitrary step and return the new value. Useful if you have
    /// pre-aggregated data.
    ///
    /// # Example
    ///
    /// ```
    /// use aoc_2020::histogram::Histogram;
    /// use std::collections::HashMap;
    ///
    /// let cups = vec![("mon", 1), ("tue", 3), /* on to next week */ ("mon", 2), ("tue", 1)];
    /// let mut hist = HashMap::new();
    /// for (day, n) in cups {
    ///     hist.increment_by(day, n);
    /// }
    /// assert_eq!(4, hist.get_count(&"tue"));
    /// assert_eq!(0, hist.get_count(&"sat"));
    /// ```
    fn increment_by(&mut self, bucket: T, step: usize) -> usize;

    /// Retrieve the value of a bucket.
    fn get_count(&self, bucket: &T) -> usize;
}

impl Histogram<usize> for Vec<usize> {
    /// Increment a bucket by the given amount, creating it (and all lower-numbered buckets) if
    /// needed.
    fn increment_by(&mut self, bucket: usize, step: usize) -> usize {
        if let None = self.get(bucket) {
            self.resize(bucket, 0);
            self.push(step);
            return step;
        } else {
            self[bucket] += step;
        }
        self[bucket]
    }

    /// Get a bucket's count; missing buckets are not created.
    fn get_count(&self, &bucket: &usize) -> usize {
        match self.get(bucket) {
            Some(&c) => c,
            None => 0,
        }
    }
}

impl<T: Eq + Hash> Histogram<T> for HashMap<T, usize> {
    /// Increment a bucket by the given amount, inserting it - and only it - if needed.
    fn increment_by(&mut self, bucket: T, step: usize) -> usize {
        let v = self.entry(bucket).or_default();
        *v = *v + step;
        *v
    }

    /// Get a bucket's count; missing buckets are not created.
    fn get_count(&self, bucket: &T) -> usize {
        match self.get(bucket) {
            Some(&c) => c,
            None => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec() {
        let mut hist = Vec::new();
        for i in 8..=21 {
            hist.increment(i % 10);
        }

        assert_eq!(hist.get_count(&8), 2);
        assert_eq!(hist, vec![2, 2, 1, 1, 1, 1, 1, 1, 2, 2])
    }

    #[test]
    fn int_map() {
        let mut hist = HashMap::new();
        let numbers = (8..=21).collect::<Vec<i32>>();
        for &i in &numbers {
            hist.increment(i % 10);
        }

        assert_eq!(hist.get_count(&2), 1);
        assert_eq!(hist.get_count(&8), 2);
        assert_eq!(hist.get_count(&99), 0);
    }

    #[test]
    fn ref_map() {
        #[derive(Eq, PartialEq, Hash)]
        struct Thing {
            name: String,
        }

        impl Thing {
            fn new(name: &str) -> Thing {
                Thing {
                    name: name.to_string(),
                }
            }
        }

        let things = vec![
            Thing::new("Barney"),
            Thing::new("Sally"),
            Thing::new("Johann"),
            Thing::new("Jackie"),
            Thing::new("Johann"),
            Thing::new("Johann"),
        ];

        let mut hist = HashMap::new();
        for t in things {
            hist.increment(t);
        }

        assert_eq!(hist.get_count(&Thing::new("Barney")), 1);
        assert_eq!(hist.get_count(&Thing::new("Johann")), 3);
    }
}
