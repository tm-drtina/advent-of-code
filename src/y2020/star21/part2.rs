use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> String {
    let mut allergens: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut ingredient_occurrences: HashMap<&str, usize> = HashMap::new();

    input.lines().for_each(|line| {
        let (ingredients_str, allergens_str) = line.split_once(" (contains ").unwrap();
        let ingredients: HashSet<&str> = ingredients_str.split(" ").collect();
        ingredients.iter().for_each(|ingredient| {
            *ingredient_occurrences.entry(ingredient).or_default() += 1;
        });
        allergens_str[0..allergens_str.len() - 1]
            .split(", ")
            .for_each(|allergen| {
                let prev_ingredients = allergens.get(allergen).map(|x| x.clone());
                match prev_ingredients {
                    Some(prev_ingredients) => {
                        allergens.insert(
                            allergen,
                            prev_ingredients
                                .intersection(&ingredients)
                                .copied()
                                .collect(),
                        );
                    }
                    None => {
                        allergens.insert(allergen, ingredients.clone());
                    }
                };
            });
    });

    let mut res: Vec<(&str, &str)> = Vec::new();

    for _ in 0..allergens.len() {
        let (allergen, ingredients) = allergens.iter().filter(|x| x.1.len() == 1).next().unwrap();
        let ingredient = *ingredients.iter().next().unwrap();
        let allergen = *allergen;
        res.push((allergen, ingredient));
        allergens.remove(allergen);
        allergens.iter_mut().for_each(|(_, a)| {
            a.remove(ingredient);
        });
    }

    res.sort_by_key(|x| x.0);

    res.iter().map(|x| x.1).join(",")
}
