use aoc_2020::histogram::Histogram;
use aoc_2020::indexer::Indexer;
use std::collections::{HashMap, HashSet, LinkedList};
use std::str::FromStr;

pub fn the_work() {
    let input = aoc_2020::read_input();
    let foods = parse_food(&input);
    let ans = solve(&foods);
    println!("{:?}", ans.0);
    println!("{:?}", ans.1);
}

fn solve(foods: &[RawFood]) -> (usize, String) {
    // I thought a trie would be better than a hashmap, but it turns
    // out the latter is about 50% faster. I guess the collection is
    // small enough and the strings short enough that the trie overhead
    // isn't justified compared to the repeated iteration to compute
    // string hashes.
    let mut ing_indexer = HashMap::new();
    let mut ing_names = HashMap::new();
    let mut al_indexer = HashMap::new();
    let mut al_names = HashMap::new();

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
            ing_names.insert(idx, ing);
            // record it in the food's ingredient set
            ing_ids.insert(idx);
            // add it to the ingredient histogram
            ing_hist.increment_bucket(idx);
        }
        // for each allergen...
        for al in &f.allergens {
            let idx = al_indexer.index_of(al);
            al_names.insert(idx, al);
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

    // sort the allergens by the number of candidate ingredients they have
    let mut al_idx_cands_pairs = al_candidates.into_iter().enumerate().collect::<Vec<_>>();
    al_idx_cands_pairs.sort_by_key(|(_, cs)| cs.len());
    println!("{:?}", al_idx_cands_pairs);

    // convert to a queue, as we may need to reprocess, depending on how they
    // end up ordering.
    let mut al_idx_cands_pairs = al_idx_cands_pairs.into_iter().collect::<LinkedList<_>>();

    // iterate through the allergens, remove all already-claimed ingredients from their
    // set of candidates, which should leave a single candidate. That's the source
    // ingredient for the allergen.
    let mut ing_to_al = HashMap::new();
    while let Some((idx, mut cs)) = al_idx_cands_pairs.pop_front() {
        for k in ing_to_al.keys() {
            cs.remove(k);
        }
        if cs.len() == 1 {
            ing_to_al.insert(*cs.iter().next().unwrap(), idx);
        } else {
            // still ambiguous
            al_idx_cands_pairs.push_back((idx, cs));
        }
    }
    println!("{:?}", ing_to_al);

    // iterate through the ingredient histogram and sum up the counts of all
    // ingredients which aren't allergen sources.
    let safe_ing_count = ing_hist
        .iter()
        .enumerate()
        .filter(|(i, _)| !ing_to_al.contains_key(i))
        .map(|(_, n)| n)
        .sum();

    // inflate the mapping into a slice of pairs of names, sort by the allergen, and
    // collect the ingredients.
    let mut al_ing_pairs = ing_to_al
        .iter()
        .map(|(ii, ai)| (al_names[ai], ing_names[ii]))
        .collect::<Vec<_>>();
    al_ing_pairs.sort();
    println!("{:?}", al_ing_pairs);
    let dangerous_ing_list = al_ing_pairs
        .iter()
        .map(|&(_, ing)| &ing[..])
        .collect::<Vec<_>>()
        .join(",");

    (safe_ing_count, dangerous_ing_list)
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
        assert_eq!((5, String::from("mxmxvkd,sqjhc,fvjkl")), solve(&foods));
    }
}
