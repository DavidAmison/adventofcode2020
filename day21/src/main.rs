use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

fn main() {
    let lines = read_in_lines("src/ingredients.txt");
    let mut ingredients_lists = Vec::new();
    let mut allergen_lists = Vec::new();

    for line in lines {
        let temp: Vec<&str> = line.split('(').collect();
        // Extract ingredients (space seprated)
        let ingredients: Vec<String> = temp[0].split_whitespace().map(|x| String::from(x)).collect();
        // Extract allergens (space and comma separated with bracket at the end which need removing)
        let allergens: Vec<String> = temp[1][9..].trim_end_matches(')').split_whitespace().map(|x| String::from(x.trim_end_matches(","))).collect();
        ingredients_lists.push(ingredients);
        allergen_lists.push(allergens);
    }

    println!("--- Part 1 ---");
    // Figure out what ingredients might be allergens
    let mut candidates: HashMap<String, Vec<String>> = HashMap::new();
    for (i, l1) in ingredients_lists.iter().enumerate() {
        for allergen in allergen_lists[i].iter() {
            if let Some(l2) = candidates.get_mut(allergen) {
                l2.retain(|i| l1.contains(i));
            } else {
                candidates.insert(allergen.clone(), l1.clone());
            }
        }
    }

    // Get a list of all the candidate ingredients (without duplicates)
    let mut all_candidate_ingredients = Vec::new();
    for l in candidates.values() {
        all_candidate_ingredients.append(&mut l.clone());
    }
    all_candidate_ingredients.sort();
    all_candidate_ingredients.dedup();

    // Get a list of all ingredients (without duplicates)
    let mut all_ingredients = Vec::new();
    for l in ingredients_lists.iter() {
        all_ingredients.append(&mut l.clone());
    }
    all_ingredients.sort();
    all_ingredients.dedup();

    // Which ingredients are not contained in the candidate ingredients list
    let mut no_allergen_ingredients = all_ingredients.clone();
    no_allergen_ingredients.retain(|i| !all_candidate_ingredients.contains(i));

    // How many times do they appear in different recipies
    let mut count = 0;
    for l in ingredients_lists.iter() {
        for ingredient in no_allergen_ingredients.iter() {
            if l.contains(ingredient) {
                count += 1;
            }
        }
    }
    println!("Part 1 answer: {}", count);

    println!("\n--- Part 2 ---");
    // Now we loop over our candidate allergens untill we reduce each allergen to 1 ingredient
    let mut taken = Vec::new();
    let mut allergens = Vec::new();
    loop {
        let mut keep_going = false;
        for (a, i) in candidates.iter_mut() {
            if i.len() == 1 && !taken.contains(&i[0]) {
                taken.push(i[0].clone());
                allergens.push((a.clone(), i[0].clone()));
                println!("{} -> {}", a, i[0]);
            } else if i.len() > 1 {
                // Trim away the taken ingredients
                i.retain(|x| !taken.contains(x));
                keep_going = true;
            }
        }
        if !keep_going {
            break;
        }
    }
    print!("\nCanonical Dangerous Ingredients List: ");

    // Sort alphabetically and 
    allergens.sort();
    for (_a, i) in allergens {
        print!("{},", i);
    }
    println!("");  
    
}

/// Read in lines of a file to a vector
/// 
/// # Arguments
/// 
/// * `filename` - String containing the filename
fn read_in_lines(filename: &str) -> Vec<String> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().flatten().collect()

}
