use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let lines = read_in_file("src/formresponses.txt");
    let groups = seperate_groups(lines);

    let mut count = 0;
    for group in groups.iter() {
        count += count_unique_responses(group);
    }
    println!("Total unique responses: {}", count);

    let mut count = 0;
    for group in groups.iter() {
        count += count_non_unique_responses(group);
    }
    println!("Total non-unique responses: {}", count);

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

fn seperate_groups(input: Vec<String>) -> Vec<Vec<String>> {
    let mut output = Vec::new();
    let mut group = Vec::new();
    for line in input {
        if line.trim().is_empty() {
            output.push(group.clone());
            group = Vec::new();
        } else {
            group.push(line);
        }
    }
    output.push(group.clone());
    output
}

fn count_unique_responses(input: &Vec<String>) -> usize {
    let mut unique_responses = HashMap::new();
    for line in input {
        for c in line.chars() {
            *unique_responses.entry(c).or_insert(1) += 1;
        }
    }
    unique_responses.len()
}

fn count_non_unique_responses(input: &Vec<String>) -> usize {
    let mut responses = HashMap::new();
    let group_size = input.len();
    for line in input {
        for c in line.chars() {
            *responses.entry(c).or_insert(0) += 1;
        }
    }

    let mut result = 0;
    for (_, count) in responses.iter() {
        if *count == group_size {
            result += 1;
        }
    }
    result
}
