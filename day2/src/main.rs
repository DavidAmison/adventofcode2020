/**
--- Day 2: Password Philosophy ---
Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.
The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.
Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.
To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.
For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.
How many passwords are valid according to their policies?

--- Part Two ---

While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.
The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.
Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
Given the same example list from above:

    1-3 a: abcde is valid: position 1 contains a and position 3 does not.
    1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
    2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.

How many passwords are valid according to the new interpretation of the policies?
**/

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let passwords = read_in_lines("src/passwords.txt");

    let mut part_one = 0;
    let mut part_two = 0;

    for password in passwords {
        let (p, policy) = parse_password(password);
        if sled_rental_policy(&p, &policy) { part_one += 1; }
        if toboggon_corp_policy(&p, &policy) { part_two += 1; }
    }
    
    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
}


/// Representation of a password policy
struct PasswordPolicy {
    character: char,
    first: u32,
    second: u32,
}

/// Representation of a password
struct Password {
    password: String,
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

/// Parse a password file string into Password and PasswordPolicy
/// 
/// # Arguments
/// 
/// * `password` - String containing the password and policy
/// 
/// # Return
/// 
/// * (Password, PasswordPolicy) as a tuple
fn parse_password(password: String) -> (Password, PasswordPolicy) {
    let temp: Vec< &str > = password.split_whitespace().collect();
    let range: Vec< &str > = temp[0].split('-').collect();
    let first = range[0].parse::<u32>().unwrap();
    let second = range[1].parse::<u32>().unwrap();
    let character = temp[1].chars().nth(0).unwrap();
    let password = temp[2];

    let p: Password = Password {
        password: String::from(password),
    };

    let policy: PasswordPolicy = PasswordPolicy {
        character,
        first,
        second,
    };

    (p, policy)
}


/// Check passwords match the sled_rental_policy
/// 
/// # Arguments
/// 
/// * password - the password to check
/// * policy - the policy details to check against
fn sled_rental_policy(p: &Password, policy: &PasswordPolicy) -> bool {
    let c: u32 = p.password.matches(policy.character).count() as u32;
    (c >= policy.first) && (c <= policy.second)
}

/// Check passwords match the sled_rental_policy
/// 
/// # Arguments
/// 
/// * password - the password to check
/// * policy - the policy details to check against
fn toboggon_corp_policy(p: &Password, policy: &PasswordPolicy) -> bool {
    let first = p.password.chars().nth(policy.first as usize - 1).unwrap();
    let second = p.password.chars().nth(policy.second as usize - 1).unwrap();

    ((policy.character == first) || (policy.character == second)) && (first != second)
}