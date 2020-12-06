use std::fmt;
use std::slice::Iter;

/// I represent a histogram as a heap-allocated, auto-expanding set of zero-indexed counts.
/// Conceptually, a `Vec<usize>` with histogram-y methods. Implementations are free to use a `T`
/// different than `usize` if it's more natural to talk about the counts via a different type.
pub trait Histogram<T> {
    /// Increment bucket `c` by one. If this is the first time `c` has been incremented, the
    /// histogram will be expanded to include it.
    fn increment(&mut self, c: T);

    /// Retrieve the value of bucket `c`. If `c` has never been incremented, zero will be returned,
    /// but the histogram will not be expanded.
    fn get_count(&self, c: T) -> usize;

    /// Return an `Iter` over the values in the Histogram, in bucket order.
    fn values(&self) -> Iter<'_, usize>;

    /// Return a slice of the values in the Histogram, in bucket order.
    fn raw(&self) -> &[usize];
}

/// I am a `Histogram` that exposes its counts directly by their indexes.
///
/// # Example
///
/// Build a histogram of the ones digit for the integers `8` to `21`.
///
/// ```
/// use aoc_2020::histogram::{VecHistogram, Histogram};
///
/// let mut hist = VecHistogram::new();
/// for i in 8..=21 {
///     hist.increment(i % 10);
/// }
///
/// assert_eq!(hist.get_count(8), 2);
/// assert_eq!(hist.raw(), vec![2, 2, 1, 1, 1, 1, 1, 1, 2, 2])
/// ```
#[derive(Debug)]
pub struct VecHistogram {
    values: Vec<usize>,
}

impl VecHistogram {
    pub fn new() -> VecHistogram {
        VecHistogram { values: Vec::new() }
    }
}

impl Histogram<usize> for VecHistogram {
    fn increment(&mut self, cat_idx: usize) {
        if let None = self.values.get(cat_idx) {
            self.values.resize(cat_idx, 0);
            self.values.push(1);
        } else {
            self.values[cat_idx] += 1;
        }
    }

    fn get_count(&self, cat_idx: usize) -> usize {
        match self.values.get(cat_idx) {
            Some(&c) => c,
            None => 0,
        }
    }

    fn values(&self) -> Iter<'_, usize> {
        self.values.iter()
    }

    fn raw(&self) -> &[usize] {
        &self.values
    }
}

/// I am `Histogram` which exposes its counts based on a `Fn(&T) -> usize` mapping, which is useful
/// to hide the indexes inside a `Histogram`.
///
/// # Examples
///
/// Build a histogram of the ones digit for the integers `8` to `21`. In particular note that a
/// count is not retrieved via a bucket index, but rather via something that would end up in that
/// bucket.
///
/// ```
/// use aoc_2020::histogram::{MappedHistogram, Histogram};
///
/// fn to_idx(i: &i32) -> usize {
///     (i % 10) as usize
/// }
///
/// let mut hist = MappedHistogram::new(&to_idx);
/// let numbers = (8..=21).collect::<Vec<i32>>();
/// for i in &numbers {
///     hist.increment(i);
/// }
///
/// assert_eq!(hist.get_count(&58), 2);
/// assert_eq!(hist.raw(), vec![2, 2, 1, 1, 1, 1, 1, 1, 2, 2])
/// ```
pub struct MappedHistogram<'a, T> {
    hist: VecHistogram,
    to_index: &'a dyn Fn(&T) -> usize,
}

// Fn isn't Debug, so implement manually...
impl<T> fmt::Debug for MappedHistogram<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MappedHistogram")
            .field("values", &self.hist.values)
            .finish()
    }
}

impl<'a, T> MappedHistogram<'a, T> {
    pub fn new<F>(f: &'a F) -> MappedHistogram<'a, T>
    where
        F: Fn(&T) -> usize,
    {
        MappedHistogram {
            hist: VecHistogram::new(),
            to_index: f,
        }
    }
}

impl<T> Histogram<&T> for MappedHistogram<'_, T> {
    fn increment(&mut self, cat: &T) {
        self.hist.increment((self.to_index)(cat));
    }

    fn get_count(&self, cat: &T) -> usize {
        self.hist.get_count((self.to_index)(cat))
    }

    fn values(&self) -> Iter<'_, usize> {
        self.hist.values()
    }

    fn raw(&self) -> &[usize] {
        &self.hist.raw()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn of_ints() {
        fn to_idx(i: &i32) -> usize {
            (i % 10) as usize
        }

        let mut hist = MappedHistogram::new(&to_idx);
        let numbers = (8..=21).collect::<Vec<i32>>();
        for i in &numbers {
            hist.increment(i);
        }

        assert_eq!(hist.get_count(&58), 2);
        assert_eq!(hist.raw(), vec![2, 2, 1, 1, 1, 1, 1, 1, 2, 2])
    }

    #[test]
    fn raw_indexes() {
        let mut hist = VecHistogram::new();
        for i in 8..=21 {
            hist.increment(i % 10);
        }

        assert_eq!(hist.get_count(8), 2);
        assert_eq!(hist.raw(), vec![2, 2, 1, 1, 1, 1, 1, 1, 2, 2])
    }

    #[test]
    fn of_refs() {
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

        fn to_idx_len(t: &Thing) -> usize {
            t.name.len()
        }

        fn to_idx_letter(&c: &char) -> usize {
            (c as usize) - ('a' as usize)
        }

        fn to_idx_name_char_2(t: &Thing) -> usize {
            to_idx_letter(&t.name.chars().nth(1).unwrap())
        }

        let things = vec![
            Thing::new("Barney"),
            Thing::new("Sally"),
            Thing::new("Johann"),
            Thing::new("Jackie"),
        ];
        let mut len_hist = MappedHistogram::new(&to_idx_len);
        let mut sec_hist_indirect = VecHistogram::new();
        let mut sec_hist_direct = MappedHistogram::new(&to_idx_name_char_2);
        for t in &things {
            len_hist.increment(t);
            sec_hist_indirect.increment(to_idx_letter(&t.name.chars().nth(1).unwrap()));
            sec_hist_direct.increment(t);
        }
        println!("{:?}", len_hist);
        assert_eq!(len_hist.raw(), vec![0, 0, 0, 0, 0, 1, 3]);
        println!("{:?}", sec_hist_indirect);
        assert_eq!(sec_hist_indirect.get_count(0), 3); // the a's
        assert_eq!(sec_hist_indirect.get_count(14), 1); // the o's
        println!("{:?}", sec_hist_direct);
        assert_eq!(sec_hist_direct.get_count(&Thing::new("Louis")), 1); // the o's
    }
}
