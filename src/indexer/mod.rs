use std::collections::HashMap;

/// I provide unique indexing of some `K`, useful for representing a set as an array.
///
/// # Examples:
///
/// ```
/// use std::collections::HashMap;
/// use aoc_2020::indexer::Indexer;
///
/// let mut indexer = HashMap::new();
/// assert_eq!(0, indexer.index_of("a"));
/// assert_eq!(1, indexer.index_of("b"));
/// assert_eq!(0, indexer.index_of("a"));
/// ```
pub trait Indexer<K> {
    fn index_of(&mut self, needle: K) -> usize;
}

impl Indexer<&str> for HashMap<String, usize> {
    fn index_of(&mut self, needle: &str) -> usize {
        match self.get(needle) {
            Some(&idx) => idx,
            None => {
                let idx = self.len();
                self.insert(needle.to_owned(), idx);
                idx
            }
        }
    }
}
