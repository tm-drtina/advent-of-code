use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> usize {
    let mut allergens: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut ingredient_occurrences: HashMap<&str, usize> = HashMap::new();

    input.lines().for_each(|line| {
        let (ingredients_str, allergens_str) = line.split_once(" (contains ").unwrap();
        let ingredients: HashSet<&str> = ingredients_str.split(' ').collect();
        for ingredient in &ingredients {
            *ingredient_occurrences.entry(ingredient).or_default() += 1;
        }
        allergens_str[0..allergens_str.len() - 1]
            .split(", ")
            .for_each(|allergen| {
                let prev_ingredients = allergens.get(allergen).cloned();
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
                }
            });
    });

    ingredient_occurrences
        .into_iter()
        .filter(|(key, _)| allergens.values().all(|x| !x.contains(key)))
        .map(|(_, value)| value)
        .sum()
}
