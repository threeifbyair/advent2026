use crate::Args;
use std::collections::{HashMap, HashSet};
use std::io;

type IngredientSet<'a> = HashSet<&'a str>;
type IngredientMap<'a> = HashMap<&'a str, IngredientSet<'a>>;

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    fn recurse_ingredients<'a>(ingredient_map: &IngredientMap<'a>, ingredients: &IngredientSet<'a>, contained_ingredients: &mut IngredientSet<'a>, verbose: bool) {
        if verbose {
            println!("Ingredients to check: {:?}", ingredients);
        }
        for ingredient in ingredients {
            if verbose {
                println!("Checking ingredient: {} mapping to {:?}", ingredient, ingredient_map.get(ingredient));
            }
            if contained_ingredients.contains(ingredient) {
                if verbose {
                    println!("Already have ingredient: {}", ingredient);
                }
            } else {
                contained_ingredients.insert(ingredient);
                if let Some(sub_ingredients) = ingredient_map.get(ingredient) {
                    recurse_ingredients(ingredient_map, sub_ingredients, contained_ingredients, verbose);
                }
            }
        }
    }

    let mut ingredient_map: IngredientMap = IngredientMap::new();
    for line in data {
        let (key, values) = line.split_once(':').unwrap();
        ingredient_map.insert(key, IngredientSet::from_iter(values.split(',')));
    }
    let mut all_ingredients: IngredientMap = IngredientMap::new();
    for (key, values) in &ingredient_map {
        let mut contained_ingredients: IngredientSet = IngredientSet::new();
        recurse_ingredients(&ingredient_map, values, &mut contained_ingredients, args.verbose);

        contained_ingredients.remove(key);
        if args.verbose {
            println!("Ingredients in {key}: {:?}\n", contained_ingredients);
        }
        all_ingredients.insert(key, contained_ingredients);
    }

    if args.part_three {
        let count = all_ingredients
            .values()
            .filter(|v| v.iter().filter(|s| s.contains("NULL")).count() >= 2)
            .count();
        Ok(format!("Total ingredients containing two NULLs: {count}"))
    } else if args.part_two {
        let count = all_ingredients.values().filter(|v| v.contains("NULL")).count();
        Ok(format!("Total ingredients containing NULL: {count}"))
    } else {
        if args.verbose {
            println!("Contained ingredients: {:?}", all_ingredients["NULL"]);
        }

        Ok(format!("Total ingredients: {}", all_ingredients["NULL"].len()))
    }
}
