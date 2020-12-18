use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = read_in_lines("src/homework.txt");
    
    let mut total1 = 0;
    let mut total2 = 0;
    for line in lines {
        total1 += eval_in_order(&line);
        total2 += eval_with_precedence(&line);
    }
    
    println!("\n--- Part 1 ---");
    println!("Sum of all answers = {}", total1);

    println!("\n--- Part 2 ---");
    println!("Sum of all answers = {}", total2);

    // eval_with_precedence(&String::from("5 + (8 * 4 + 9 + 3 * 4 * 3)"));
    // println!("{}", eval_with_precedence(&String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")));
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

/// Evaluate the input expression with no operator precendence except brackets
/// Note: The only valid operators are brackets '(...)', addition '+' and multiplication '*'
/// 
/// # Arguments
/// 
/// * `exp` the expression to evaluate
fn eval_in_order(exp: &String) -> u64 {
    // First we evaluate all expressions inside brackets recursively
    let mut collapsed_brackets = String::new();
    let mut bracket_num = 0;
    let mut substring = String::new();
    for c in exp.chars() {
        if bracket_num > 0 {
            match c {
                '(' => {
                    bracket_num += 1;
                    substring.push(c);
                }
                ')' => {
                    bracket_num -= 1;
                    if bracket_num != 0 {
                        substring.push(c);
                    } else {
                        // Evaluate expression inside brackets
                        collapsed_brackets.push_str(&format!("{}", eval_in_order(&substring)));
                        substring = String::from("");
                    }
                }
                _ => substring.push(c),
            };
        } else {
            match c {
                '(' => bracket_num += 1,
                _ => collapsed_brackets.push(c),
            };
        }
    }

    let mut result = 0;
    let mut operator: char = '+';
    for term in collapsed_brackets.split_whitespace() {
        match term {
            "+" => operator = '+',
            "*" => operator = '*',
            _ => {
                let x = term.parse::<u64>().unwrap();
                match operator {
                    '+' => result += x,
                    '*' => result *= x,
                    _ => panic!("Unrecognised operator: {}", operator),
                }
            }
        }
    }
    result
}

/// Evaluate the input expression with operator precedence of (...), +, *
/// Note: The only valid operators are brackets '(...)', addition '+' and multiplication '*'
/// 
/// # Arguments
/// 
/// * `exp` the expression to evaluate
fn eval_with_precedence(exp: &String) -> u64 {
    // First we evaluate all expressions inside brackets recursively
    let mut collapsed_brackets = String::new();
    let mut bracket_num = 0;
    let mut substring = String::new();
    for c in exp.chars() {
        if bracket_num > 0 {
            match c {
                '(' => {
                    bracket_num += 1;
                    substring.push(c);
                }
                ')' => {
                    bracket_num -= 1;
                    if bracket_num != 0 {
                        substring.push(c);
                    } else {
                        // Evaluate expression inside brackets
                        collapsed_brackets.push_str(&format!("{}", eval_with_precedence(&substring)));
                        substring = String::from("");
                    }
                }
                _ => substring.push(c),
            };
        } else {
            match c {
                '(' => bracket_num += 1,
                _ => collapsed_brackets.push(c),
            };
        }
    }

    // Since we are only concerned with two operators (+ and *) we can
    // simply store a subtotal between mutliplication operators that
    // is the sum of all terms between.
    let mut sub_total = 0;
    let mut result = 1;
    for term in collapsed_brackets.split_whitespace() {
        match term {
            "+" => (),
            "*" => {
                result *= sub_total;
                sub_total = 0;
            }
            _ => {
                sub_total += term.parse::<u64>().unwrap();
            }
        };
    }
    result * sub_total
}
