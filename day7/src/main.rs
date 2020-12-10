use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {

    let mut search_terms = vec!(String::from("shiny gold"));
    let mut results = Vec::new();
    loop {
        let rules = read_in_file("src/bagrules.txt");
        let mut bags: Vec<Bag> = Vec::new();
        for rule in rules {
            bags.push( interpret_rule(&rule) );
        }
    
        search_terms = search_bags(bags, &search_terms);

        if search_terms.len() == 0 {
            break;
        }

        for bag in search_terms.iter() {
            if !results.contains(bag) {
                results.push(bag.clone());
            }
        }
        println!("RESULTS: {:?}", results.len());
    }

    // Traverse the map...
    let rules = read_in_file("src/bagrules.txt");
    let mut bags: HashMap<String, Bag> = HashMap::new();
    for rule in rules {
        let bag = interpret_rule(&rule);
        bags.insert(bag.bag_description.clone(), bag);
    }

    // -1 beacause the calculation includes the gold bag
    println!("The shiny gold bag must contain {} bags", count_bag_contents(&bags, String::from("shiny gold")) - 1);
}

/// Read in lines of a file to a vector
/// 
/// # Arguments
/// 
/// * `filename` - String containing the filename
fn read_in_file(filename: &str) -> Vec<String> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().flatten().collect()
}

/// Represents a bag with its rules
#[derive(Debug)]
struct Bag {
    bag_description: String,
    contents: Option< Vec<BagRule> >,
}

/// Represents a rule that a bag must conform to
#[derive(Debug)]
struct BagRule {
    description: String,
    number: u32
}

/// Interpret the rule of the form '[bag description] contains [number] [description] bag[s], ...
fn interpret_rule(rule: &str) -> Bag {
    // Rule is of the form '[bag description] contains [number] [description] bag[s], ...
    
    // Remove all words bag, bags and full stops
    let stripped = rule.replace(" bags", " ").replace(" bag", " ").replace(".", "");
    // First we will split on the word 'contains'
    let temp: Vec<&str> = stripped.split("contain").collect();
    let bag_description = String::from(temp[0].trim());
    let contents_unparsed = temp[1].trim().split(',');
    let mut holds: Vec<BagRule> = Vec::new();
    for bag in contents_unparsed {
        // Ignore bags that contain no other bag
        if bag == "no other" {
            break
        }
        let words: Vec<&str> = bag.split_whitespace().collect();
        let number = words[0].parse::<u32>().unwrap();
        let mut description: String = String::from(words[1]);
        description.push_str(" ");
        description.push_str(words[2]);
        holds.push(BagRule { description, number, })
    }

    if holds.len() == 0 {
        Bag {
            bag_description,
            contents: None,
        }
    } else {
        Bag {
            bag_description,
            contents: Some(holds),
        }
    }
    // println!("DESC: {}, CONTENTS: {:?}", description, contents);
}

/// Return a list of bags that contain those in the search_items
fn search_bags(bags: Vec<Bag>, search_items: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    for bag in bags {
        if bag.contents.is_some() {
            for rule in bag.contents.unwrap() {
                if search_items.contains(&rule.description) {
                    if !result.contains(&bag.bag_description) {
                        result.push(bag.bag_description);
                        break;
                    }
                }
            }
        }
    }
    result
}

/// Recursive search of a bag map to find the number of bags contained
fn count_bag_contents(bags: &HashMap<String, Bag>, bag: String) -> u32 {
    let mut total = 1;
    if let Some(bag_contents) = &bags.get(&bag).unwrap().contents {
        for item in bag_contents {
            total += item.number * count_bag_contents(bags, item.description.clone());
        }
    }
    total
}
