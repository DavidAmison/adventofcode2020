mod rules;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut rs = HashMap::new();
    for line in read_in_lines("src/rules.txt") {
        let temp: Vec<String> = line.split(':').map(|x| String::from(x.trim())).collect();
        let id = temp[0].parse::<usize>().unwrap();
        let rule = String::from(temp[1].trim_matches('"'));
        rs.insert(id, rules::Rule{id, rule});
    }
    let r = Regex::new(&format!(r"{}{}{}", "\\b", rules::expand_rule(rs.get(&0).unwrap(), &rs), "\\b")).unwrap();
    // println!("{:?}", r);
    let mut valid = 0;
    for message in read_in_lines("src/messages.txt") {
        if r.is_match(&message) {
            valid += 1;
        }
    }
    println!("{}", valid);
    


    /*
    let combinations =  rules::expand_rule(rs.get(&0).unwrap(), &rs);
    // println!("{:?}", combinations);

    let mut valid = 0;
    for message in read_in_lines("src/messages.txt") {
        if !combinations.contains(&message) {
            // println!("{} is invalid", message);
        } else {
            valid += 1;
            // println!("{} is valid", message);
        }
    }
    println!("{}", valid);
    */

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
