/**
 * --- Day 6: Custom Customs ---
 * 
 * As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are distributed to the passengers.
 * The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is identify the questions for which anyone in your group answers "yes". Since your group is just you, this doesn't take very long.
 * However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help. For each of the people in their group, you write down the questions for which they answer "yes", one per line. For example:
 * 
 * abcx
 * abcy
 * abcz
 * 
 * In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z. (Duplicate answers to the same question don't count extra; each question counts at most once.)
 * Another group asks for your help, then another, and eventually you've collected answers from every group on the plane (your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers are on a single line. For example:
 * 
 * abc
 * 
 * a
 * b
 * c
 * 
 * ab
 * ac
 * 
 * a
 * a
 * a
 * a
 * 
 * b
 * 
 * This list represents answers from five groups:
 *     The first group contains one person who answered "yes" to 3 questions: a, b, and c.
 *     The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
 *     The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
 *     The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
 *     The last group contains one person who answered "yes" to only 1 question, b.
 * 
 * In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.
 * 
 * For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?
 * 
 * --- Part Two ---
 * 
 * As you finish the last group's customs declaration, you notice that you misread one word in the instructions:
 * You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!
 * Using the same example as above:
 * 
 * abc
 * 
 * a
 * b
 * c
 * 
 * ab
 * ac
 * 
 * a
 * a
 * a
 * a
 * 
 * b
 * 
 * This list represents answers from five groups:
 * 
 *     In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
 *     In the second group, there is no question to which everyone answered "yes".
 *     In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
 *     In the fourth group, everyone answered yes to only 1 question, a.
 *     In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
 * 
 * In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.
 * 
 * For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?
 */

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
