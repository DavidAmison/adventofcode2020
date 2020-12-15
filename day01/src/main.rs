/*
 * --- Day 1: Report Repair ---
 * 
 * After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.
 * 
 * The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.
 * 
 * To save your vacation, you need to get all fifty stars by December 25th.
 * 
 * Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
 * 
 * Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.
 * 
 * Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
 * 
 * For example, suppose your expense report contained the following:
 * 
 * 1721
 * 979
 * 366
 * 299
 * 675
 * 1456
 * 
 * In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.
 * 
 * Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
 * 
 * --- Part Two ---
 * 
 * The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.
 * 
 * Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.
 * 
 * In your expense report, what is the product of the three entries that sum to 2020?
 * 
 */


use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let lines = read_in_lines(filename);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut values = Vec::new();
    for line in lines {
        values.push(line.parse::<u32>().unwrap());
    }
    values.sort();

    let (x, y) = find_sum_from_two_values(2020, &values);
    println!("{} * {} = {}", x, y, x * y);

    let (x, y, z) = find_sum_from_three_values(2020, &values);
    println!("{} * {} * {} = {}", x, y, z, x * y * z);
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
/// Note: Passed vector must be sorted smallest to largest
/// 
/// # Arguments
/// 
/// * `target` - The target value
/// * `sroted_values` - A sorted vector (smalles to largest) containing the search space
fn find_sum_from_two_values(target: u32, sorted_values: &Vec<u32>) -> (u32, u32) {
    // Iterate over all numbers and find the two that sum to target value
    // We can break once sum is greater than target because numbers are
    // ordered smallest to largest
    for (i, x) in sorted_values[0..].iter().enumerate() {
        if *x > target { break; }
        for (_, y) in sorted_values[i..].iter().enumerate() {
            if x + y  == target {
                    return (*x, *y)
            } else if x + y > target { break; }
        }
    }
    (0, 0)
}


/// Find three values in the passed vector that add to the target value
/// Note: Passed vector must be sorted smallest to largest
/// 
/// # Arguments
/// 
/// * `target` - The target value
/// * `sroted_values` - A sorted vector (smalles to largest) containing the search space
fn find_sum_from_three_values(target: u32, sorted_values: &Vec<u32>) -> (u32, u32, u32) {
    // Iterate over all numbers and find the two that sum to target value
    // We can break once sum is greater than target because numbers are
    // ordered smallest to largest
    for (i, x) in sorted_values[0..].iter().enumerate() {
        if *x > target { break; }
        for (j, y) in sorted_values[i..].iter().enumerate() {
            if x + y > target { break; } 
            for (_, z) in sorted_values[j..].iter().enumerate() {
                if x + y + z == target {
                    return (*x, *y, *z)
                } else if x + y + z > target { break; }
            }
        }
    }
    (0, 0, 0)
}


