use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut rules = read_in_rules("src/rules.txt");
    // println!("{:#?}", &rules);

    let tickets = read_in_comma_separated_values("src/tickets.txt");

    let mut invalid_sum = 0;
    let mut valid_tickets = Vec::new();
    for (_, ticket) in tickets.iter().enumerate() {
        let mut ticket_valid = true;
        for value in ticket {
            let mut value_valid = false;
            for rule in &rules {
                if rule.check(*value) {
                    value_valid = true;
                    break;
                }
            }
            if !value_valid {
                invalid_sum += value;
                ticket_valid = false;
            }
        }
        if ticket_valid {
            valid_tickets.push(ticket.clone());
        }
    }
    println!("Sum of invalid ticket values is {}", invalid_sum);

    // Number of fields
    let mut possible_fields = Vec::new();
    for rule in &rules {
        let mut valid_fields = Vec::new();
        for i in 0..rules.len() {
            let mut flag = true;
            for j in 0..valid_tickets.len() {
                if !rule.check(valid_tickets[j][i]) {
                    flag = false;
                }
                // flag = flag && rule.check(tickets[j][i]);
            }
            if flag {
                // println!("Field [{}] - valid index [{}]", rule.field_name, i);
                valid_fields.push(i);
            }
        }
        possible_fields.push(valid_fields);
    }
    // println!("{:#?}", possible_fields);

    let mut taken = Vec::new();
    let mut keep_reducing = true;
    while keep_reducing {
        keep_reducing = false;
        let mut possible_fields_new = Vec::new();
        for (i, valid_list) in possible_fields.clone().iter().enumerate() {
            let mut new_valid_list = Vec::new();
            if valid_list.len() > 1 {
                keep_reducing = true;                
            }
            for value in valid_list {
                if !taken.contains(value) {
                    new_valid_list.push(*value);
                }
            }
            if new_valid_list.len() == 1 {
                println!("{:?} is index {}", rules[i].field_name, new_valid_list[0]);
                taken.push(new_valid_list[0]);
                rules[i].index = new_valid_list[0];
            }
            possible_fields_new.push(new_valid_list);
        }
        possible_fields = possible_fields_new;
    }
    // println!("{:#?}", &rules);

    let my_ticket = [79,67,101,89,131,107,139,113,127,83,137,53,71,149,73,97,59,61,109,103];
    let mut result: u64 = 1;
    for rule in &rules {
        if rule.field_name.contains("departure") {
            println!("{} = {}", rule.field_name, my_ticket[rule.index]);
            result *= my_ticket[rule.index];         
        }
    }
    println!("{}", result);

}


/// Read in comma separated values to a vector
/// 
/// # Arguments
/// 
/// * `filename` - String containing the filename
/// 
/// # Returns
/// 
/// * a vector of u32 values containing the comma separated parameters 
fn read_in_comma_separated_values(filename: &str) -> Vec< Vec<u32> > {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    let mut result = Vec::new();
    for line in reader.lines() {
        let values: Vec<u32> = line.unwrap().split(',').map(|s| s.parse::<u32>().unwrap()).collect();
        result.push(values);
    }
    result
}

#[derive(Debug)]
struct TicketRule {
    field_name: String,
    valid_ranges: Vec<Range>,
    index: usize,
}

impl TicketRule {
    /// Check te given value is contained in the valid ranges of the rule
    /// 
    /// # Arguments
    /// 
    /// `x` the value to check
    fn check(&self, x: u32) -> bool {
        for range in self.valid_ranges.iter() {
            if range.contains(x) {
                return true
            }
        }
        false
    }
}

/// Define an inclusive range of values
#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}

impl Range {
    /// Check if a value is contained by the range
    /// 
    /// # Arguments
    /// 
    /// `x` the value to check
    fn contains(&self, x: u32) -> bool {
        (self.min <= x) && (x <= self.max)
    }
}

fn read_in_rules(filename: &str) -> Vec< TicketRule > {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rules = Vec::new();
    for line in reader.lines() {
        if let Ok(l) = line {
            let temp1: Vec<&str> = l.split(':').collect();
            let field_name = String::from(temp1[0].trim());

            let mut valid_ranges = Vec::new();
            let temp2: Vec<&str> = temp1[1].trim().split("or").collect();
            for range in temp2 {
                let temp3: Vec<&str> = range.trim().split('-').collect();
                let min = temp3[0].parse::<u32>().unwrap();
                let max = temp3[1].parse::<u32>().unwrap();
                valid_ranges.push( Range{min, max} );
            }
            rules.push( TicketRule{field_name, valid_ranges, index: 0 } );
        }
    }
    rules
}
