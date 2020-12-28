use std::str::FromStr;

pub fn the_work() {
    let input = aoc_2020::read_input();
    let foods = parse_food(&input);
    println!("{:?}", part_one(&foods));
}

fn part_one(foods: &[Food]) -> usize {
    foods.len()
}

fn parse_food(input: &str) -> Vec<Food> {
    input.trim().lines().map(|l| l.parse().unwrap()).collect()
}

type Ingredient = String;
type Allergen = String;
type Ings = Vec<Ingredient>;
type Allgs = Vec<Allergen>;

#[derive(Eq, PartialEq, Debug)]
struct Food {
    ingredients: Ings,
    allergens: Allgs,
}

impl Food {
    fn new(ingredients: Ings, allergens: Allgs) -> Food {
        Food {
            ingredients,
            allergens,
        }
    }
}

impl FromStr for Food {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut on_allergens = false;
        let mut ings= Vec::new();
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
        Ok(Food::new(ings, allgs))
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
