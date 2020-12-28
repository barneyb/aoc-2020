use aoc_2020::histogram::Histogram;
use aoc_2020::indexer::Indexer;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub fn the_work() {
    let input = aoc_2020::read_input();
    let foods = parse_food(&input);
    println!("{:?}", part_one(&foods));
}

fn part_one(foods: &[RawFood]) -> usize {
    // I thought a trie would be better than a hashmap, but it turns
    // out the latter is about 50% faster. I guess the collection is
    // small enough and the strings short enough that the trie overhead
    // isn't justified compared to the repeated iteration to compute
    // string hashes.
    let mut ing_indexer = HashMap::new();
    let mut al_indexer = HashMap::new();

    // per-ingredient counts of instances
    let mut ing_hist = Vec::new();
    // per-allergen sets of candidate ingredients
    let mut al_candidates = Vec::new();

    // for each food...
    for f in foods {
        let mut ing_ids = HashSet::new();
        // for each ingredient...
        for ing in &f.ingredients {
            let idx = ing_indexer.index_of(ing);
            // record it in the food's ingredient set
            ing_ids.insert(idx);
            // add it to the ingredient histogram
            ing_hist.increment_bucket(idx);
        }
        // for each allergen...
        for al in &f.allergens {
            let idx = al_indexer.index_of(al);
            // if it's the first occurrence
            if al_candidates.len() == idx {
                // all of this food's ingredients are candidates
                al_candidates.push(ing_ids.clone());
            } else {
                // the intersection of this food's ingredients w/ history remain candidates
                al_candidates[idx] = al_candidates[idx].intersection(&ing_ids).cloned().collect();
            }
        }
    }

    // the rules say every allergen has a single source ingredient, which implies
    // the union of all allergens' candidate sets must match the allergen count.
    let mut allergen_sources = HashSet::new();
    for cands in &al_candidates {
        allergen_sources = allergen_sources.union(cands).cloned().collect();
    }
    assert_eq!(allergen_sources.len(), al_candidates.len());
    assert_eq!(allergen_sources.len(), al_indexer.len());

    // now iterate through the ingredient histogram and sum up the counts of all
    // ingredients which aren't allergen sources.
    ing_hist
        .iter()
        .enumerate()
        .filter(|(i, _)| !allergen_sources.contains(i))
        .map(|(_, n)| n)
        .sum()
}

fn parse_food(input: &str) -> Vec<RawFood> {
    input.trim().lines().map(|l| l.parse().unwrap()).collect()
}

#[derive(Eq, PartialEq, Debug)]
struct RawFood {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl RawFood {
    fn new(ingredients: Vec<String>, allergens: Vec<String>) -> RawFood {
        RawFood {
            ingredients,
            allergens,
        }
    }
}

impl FromStr for RawFood {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut on_allergens = false;
        let mut ings = Vec::new();
        let mut allgs = Vec::new();
        for s in str.split(' ') {
            if on_allergens {
                // char::is_numeric(char), but char::is_ascii_punctuation(&char)? Huh?
                allgs.push(
                    s.trim_end_matches(|c| char::is_ascii_punctuation(&c))
                        .to_string(),
                )
            } else if s == "(contains" {
                on_allergens = true
            } else {
                ings.push(s.to_string())
            }
        }
        Ok(RawFood::new(ings, allgs))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn example_one() {
        let foods = parse_food(&EXAMPLE_ONE);
        for f in &foods {
            println!("{:?}", f)
        }
        assert_eq!(5, part_one(&foods));
    }
}
