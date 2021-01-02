mod rules;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut rs = rules::read_in_rules("src/rules.txt");

    println!("--- Part 1 ---");
    let r0 = rs.get(&0).unwrap();
    let mut valid = 0;
    for message in read_in_lines("src/messages.txt") {
        let result = (*r0).evaluate(&message, &rs);
        if result.contains(& Some(String::from(""))) {
            // println!("{} is valid", message);  
            valid += 1;          
        } else {
            // println!("{} is invalid", message);
        }
    }
    println!("{} messages are valid", valid);

    println!("\n--- Part 2 ---");
    // Modify the rules (8 and 11) to create a recursive rule
    let r8 = rules::Rule { rule_type: rules::RuleType::Compound, rule: String::from("42 | 42 8") };
    let r11 = rules::Rule { rule_type: rules::RuleType::Compound, rule: String::from("42 31 | 42 11 31") };
    rs.insert(8, r8);
    rs.insert(11, r11);
    let r0 = rs.get(&0).unwrap();
    valid = 0;
    for message in read_in_lines("src/messages.txt") {
        let result = (*r0).evaluate(&message, &rs);
        if result.contains(& Some(String::from(""))) {
            // println!("{} is valid", message);  
            valid += 1;          
        } else {
            // println!("{} is invalid", message);
        }
    }
    println!("{} messages are valid", valid);
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
