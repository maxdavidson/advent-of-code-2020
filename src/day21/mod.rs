use std::collections::{hash_map::Entry, HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

impl<'a> Food<'a> {
    fn parse(value: &'a str) -> Option<Self> {
        lazy_static! {
            static ref RECIPE_RE: Regex =
                Regex::new(r"^(?P<ingredients>.+) \(contains (?P<allergens>.+)\)$").unwrap();
        }

        let caps = RECIPE_RE.captures(value)?;

        let ingredients = caps
            .name("ingredients")
            .unwrap()
            .as_str()
            .split(' ')
            .collect();

        let allergens = caps
            .name("allergens")
            .unwrap()
            .as_str()
            .split(", ")
            .collect();

        Some(Self {
            ingredients,
            allergens,
        })
    }
}

fn find_dangerous_ingredients_helper<'a>(
    mut allergen_ingredients: Vec<(&'a str, HashSet<&'a str>)>,
) -> Option<impl Iterator<Item = &'a str>> {
    if allergen_ingredients
        .iter()
        .any(|(_, ingredients)| ingredients.is_empty())
    {
        None
    } else if allergen_ingredients
        .iter()
        .all(|(_, ingredients)| ingredients.len() == 1)
    {
        allergen_ingredients.sort_unstable_by_key(|(allergen, _)| *allergen);

        Some(
            allergen_ingredients
                .into_iter()
                .flat_map(|(_, ingredient)| ingredient.into_iter()),
        )
    } else {
        allergen_ingredients.sort_unstable_by_key(|(_, ingredients)| ingredients.len());

        allergen_ingredients
            .iter()
            .find_map(|(allergen, ingredients)| {
                if ingredients.len() == 1 {
                    None
                } else {
                    ingredients.iter().find_map(|ingredient| {
                        let mut next_allergen_ingredients = allergen_ingredients.clone();

                        for (other_allergen, ingredients) in next_allergen_ingredients.iter_mut() {
                            if allergen != other_allergen {
                                ingredients.remove(ingredient);
                            }
                        }

                        find_dangerous_ingredients_helper(next_allergen_ingredients)
                    })
                }
            });

        None
    }
}

fn find_dangerous_ingredients<'a, 'b: 'a>(
    foods: impl IntoIterator<Item = &'a Food<'b>>,
) -> Option<impl Iterator<Item = &'a str>> {
    let mut allergen_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();

    // Intersect the sets of possible ingredients for each allergen
    for food in foods.into_iter() {
        for allergen in food.allergens.iter() {
            match allergen_ingredients.entry(allergen) {
                Entry::Vacant(entry) => {
                    entry.insert(food.ingredients.clone());
                }
                Entry::Occupied(mut entry) => {
                    entry
                        .get_mut()
                        .retain(|ingredient| food.ingredients.contains(ingredient));

                    // This shouldn't happen
                    if entry.get().is_empty() {
                        return None;
                    }
                }
            }
        }
    }

    let mut allergen_ingredients: Vec<(&str, HashSet<&str>)> =
        allergen_ingredients.into_iter().collect();

    let mut visited_allergens: HashSet<&str> = HashSet::new();

    // While there is an allergen with only one possible ingredient, remove that ingredient from all other allergens
    while let Some((allergen, ingredient)) =
        allergen_ingredients
            .iter()
            .find_map(|(allergen, ingredients)| {
                if !visited_allergens.contains(allergen) && ingredients.len() == 1 {
                    Some((*allergen, ingredients.iter().copied().next()?))
                } else {
                    None
                }
            })
    {
        visited_allergens.insert(allergen);

        for (other_allergen, ingredients) in allergen_ingredients.iter_mut() {
            if allergen != *other_allergen {
                ingredients.remove(ingredient);
            }
        }
    }

    find_dangerous_ingredients_helper(allergen_ingredients)
}

pub fn part1(input: &str) -> usize {
    let foods: Vec<Food> = input.lines().filter_map(Food::parse).collect();

    let all_ingredients: HashSet<&str> = foods
        .iter()
        .flat_map(|food| food.ingredients.iter().copied())
        .collect();

    let allergen_ingredients: HashSet<&str> = find_dangerous_ingredients(&foods)
        .expect("No solution found!")
        .collect();

    let safe_ingredients = all_ingredients
        .into_iter()
        .filter(|ingredient| !allergen_ingredients.contains(ingredient));

    safe_ingredients
        .map(|ingredient| {
            foods
                .iter()
                .filter(|food| food.ingredients.contains(ingredient))
                .count()
        })
        .sum()
}

pub fn part2(input: &str) -> String {
    let foods: Vec<Food> = input.lines().filter_map(Food::parse).collect();

    let allergen_ingredients = find_dangerous_ingredients(&foods).expect("No solution found!");

    allergen_ingredients.collect::<Vec<_>>().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 5);
        assert_eq!(part1(INPUT), 2072);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), "mxmxvkd,sqjhc,fvjkl");
        assert_eq!(
            part2(INPUT),
            "fdsfpg,jmvxx,lkv,cbzcgvc,kfgln,pqqks,pqrvc,lclnj"
        );
    }
}
