use std::{collections::HashSet, io::BufRead};

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn ingredients_possibly_containing_allergen(foods: &Vec<Food>, all_ingredients: &HashSet<String>, allergen: &str) -> HashSet<String> {
    let all_ingredients = all_ingredients.clone();
    foods.iter().filter(|food| food.allergens.contains(allergen)).fold(all_ingredients, |ingredients, food| {
        ingredients.intersection(&food.ingredients).cloned().collect()
    })
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut foods = Vec::new();
    let mut all_ingredients = HashSet::new();
    let mut all_allergens = HashSet::new();
    for line in reader.lines() {
        let line = line.unwrap();

        let ingredient_list = line.split(" (contains ").nth(0).unwrap();
        let mut ingredients = HashSet::new();
        for ingredient in ingredient_list.split(' ') {
            ingredients.insert(ingredient.to_string());
            all_ingredients.insert(ingredient.to_string());
        }
        
        let allergen_list = line.split(" (contains ").nth(1).unwrap().strip_suffix(')').unwrap();
        let mut allergens = HashSet::new();
        for allergen in allergen_list.split(", ") {
            allergens.insert(allergen.to_string());
            all_allergens.insert(allergen.to_string());
        }

        foods.push(Food { ingredients, allergens });
    }


    let mut result_part1 = all_ingredients.clone();
    let mut assignments = Vec::new();
    for allergen in all_allergens.iter() {
        let possibly_allergenic = ingredients_possibly_containing_allergen(&foods, &all_ingredients, allergen);
        result_part1 = result_part1.difference(&possibly_allergenic).cloned().collect();
        assignments.push((allergen.clone(), possibly_allergenic));
    }

    let mut answer_part1 = 0;
    for ingredient in result_part1 {
        for food in foods.iter() {
            if food.ingredients.contains(&ingredient) {
                answer_part1 += 1;
            }
        }
    }

    println!("{}", answer_part1);


    while assignments.iter().any(|assignment| assignment.1.len() > 1) {
        assignments = assignments.iter().map(|assignment| {
            if assignment.1.len() == 1 {
                return assignment.clone();
            }
            
            let choices = assignment.1.iter().cloned().filter(|choice| {
                assignments.iter().all(|other_assignment| {
                    !other_assignment.1.contains(choice) || other_assignment.1.len() > 1
                })
            }).collect::<HashSet<_>>();
            (assignment.0.clone(), choices)
        }).collect::<Vec<_>>();
    }

    assignments.sort_by_key(|assignment| assignment.0.clone());
    let answer_part2 = assignments.iter().map(|assignment| {
        assignment.1.iter().next().unwrap().clone()
    }).collect::<Vec<_>>().join(",");

    println!("{}", answer_part2);
}
