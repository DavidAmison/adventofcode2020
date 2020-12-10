use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let lines = read_in_lines(filename);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut values = Vec::new();
    for line in lines {
        values.push(line.parse::<u64>().unwrap());
    }


    let mut search_value = 0;
    let mut temp_vector = Vec::new();
    for x in values.clone() {
        temp_vector.insert(0, x);
        if temp_vector.len() < 27 {
            continue
        }
        if !find_sum_from_two_values(temp_vector[0], &temp_vector[1..]) {
            println!("--- Part 1 ---\nAnswer: {}", temp_vector[0]);
            search_value = temp_vector[0];
        }
    }

    let mut result = find_sum_from_contiguous_values(search_value, values.clone());
    result.sort();

    println!("--- Part 2 ---\nmin: {}, max: {}, answer: {}", result[0], result[result.len()-1], result[0] + result[result.len()-1]);
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

/// Find two values in the passed vector that add to the target value
/// 
/// # Arguments
/// 
/// * `target` - The target value
/// * `sroted_values` - A vector containing the search space
fn find_sum_from_two_values(target: u64, values: &[u64]) -> bool {
    // Iterate over all numbers and find the two that sum to target value
    // We can break once sum is greater than target because numbers are
    // ordered smallest to largest
    for (i, x) in values[0..].iter().enumerate() {
        for (_, y) in values[i..].iter().enumerate() {
            if x + y  == target {
                return true
            }
        }
    }
    false
}

fn find_sum_from_contiguous_values(target: u64, values: Vec<u64>) -> Vec<u64> {
    let mut search = Vec::new();
    let mut sum = 0;
    for x in values {
        search.insert(0, x);
        sum += x;
        while sum > target {
            sum -= search.pop().unwrap();
            if search.len() == 0 {
                break;
            }
        }
        if sum == target && search.len() > 1 {
            break;
        }
    }
    return search;
}