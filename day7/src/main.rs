/**
 * --- Day 7: Handy Haversacks ---
 * 
 * You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.
 * Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!
 * For example, consider the following rules:
 * 
 * light red bags contain 1 bright white bag, 2 muted yellow bags.
 * dark orange bags contain 3 bright white bags, 4 muted yellow bags.
 * bright white bags contain 1 shiny gold bag.
 * muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
 * shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
 * dark olive bags contain 3 faded blue bags, 4 dotted black bags.
 * vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
 * faded blue bags contain no other bags.
 * dotted black bags contain no other bags.
 * 
 * These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.
 * You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)
 * In the above rules, the following options would be available to you:
 * 
 *     A bright white bag, which can hold your shiny gold bag directly.
 *     A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
 *     A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
 *     A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
 * 
 * So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.
 * 
 * How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
 *
 * --- Part Two ---
 * 
 * It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!
 * Consider again your shiny gold bag and the rules from the above example:
 * 
 *     faded blue bags contain 0 other bags.
 *     dotted black bags contain 0 other bags.
 *     vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
 *     dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
 * 
 * So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!
 * Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!
 * 
 * Here's another example:
 * 
 * shiny gold bags contain 2 dark red bags.
 * dark red bags contain 2 dark orange bags.
 * dark orange bags contain 2 dark yellow bags.
 * dark yellow bags contain 2 dark green bags.
 * dark green bags contain 2 dark blue bags.
 * dark blue bags contain 2 dark violet bags.
 * dark violet bags contain no other bags.
 * 
 * In this example, a single shiny gold bag must contain 126 other bags.
 * 
 * How many individual bags are required inside your single shiny gold bag?
 */

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
