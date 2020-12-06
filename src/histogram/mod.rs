use std::fmt;
use std::slice::Iter;

pub struct Histogram<'a, C> {
    hist: Vec<usize>,
    to_index: &'a dyn Fn(&C) -> usize,
}

// Fn isn't Debug, so implement manually...
impl<C> fmt::Debug for Histogram<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Histogram")
            .field("values", &self.hist)
            .finish()
    }
}

impl<'a, C> Histogram<'a, C> {
    pub fn new<F>(f: &'a F) -> Histogram<'a, C>
    where
        F: Fn(&C) -> usize,
    {
        Histogram {
            hist: Vec::new(),
            to_index: f,
        }
    }

    pub fn increment(&mut self, cat: &C) {
        let idx: usize = (self.to_index)(cat);
        if let None = self.hist.get(idx) {
            self.hist.resize(idx, 0);
            self.hist.push(1);
        } else {
            self.hist[idx] += 1;
        }
    }

    pub fn get_count(&self, cat: &C) -> usize {
        self.get_count_idx((self.to_index)(cat))
    }

    pub fn get_count_idx(&self, cat_idx: usize) -> usize {
        match self.hist.get(cat_idx) {
            Some(&c) => c,
            None => 0,
        }
    }

    pub fn values(&self) -> Iter<'_, usize> {
        self.hist.iter()
    }

    pub fn raw(&self) -> &[usize] {
        &self.hist
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
        let mut hist = Histogram::new(&to_idx);
        for i in 8..=21 {
            hist.increment(&i);
        }
        println!("{:?}", hist);
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
        let mut len_hist = Histogram::new(&to_idx_len);
        let mut sec_hist_indirect = Histogram::new(&to_idx_letter);
        let mut sec_hist_direct = Histogram::new(&to_idx_name_char_2);
        for t in &things {
            len_hist.increment(t);
            sec_hist_indirect.increment(&t.name.chars().nth(1).unwrap());
            sec_hist_direct.increment(t);
        }
        println!("{:?}", len_hist);
        assert_eq!(len_hist.raw(), vec![0, 0, 0, 0, 0, 1, 3]);
        println!("{:?}", sec_hist_indirect);
        assert_eq!(sec_hist_indirect.get_count(&'o'), 1); // the o's
        assert_eq!(sec_hist_indirect.get_count(&'a'), 3); // the a's
        assert_eq!(sec_hist_indirect.get_count_idx(0), 3); // the a's
        println!("{:?}", sec_hist_direct);
        assert_eq!(sec_hist_direct.get_count(&things[2]), 1); // the o's
        assert_eq!(sec_hist_direct.get_count_idx(0), 3); // the a's
    }
}
