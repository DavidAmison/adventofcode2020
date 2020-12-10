use std::fs::File;
use std::io::{BufRead, BufReader};

type Adapter = u32;

#[derive(Debug)]
struct AdapterMap {
    adapter: Adapter,
    possible_connections: u64,
}

/// A structure for storing the differences between the chain
/// of adapters. The maximum difference considered is three and
/// minimum is one
#[derive(Debug)]
struct Differences {
    one: u32,
    two: u32,
    three: u32,
}

impl Differences {
    /// Create a new Differences struct and fill with 0 values
    fn new() -> Differences {
        Differences {
            one: 0,
            two: 0,
            three: 0,
        }
    }

    /// Increment one of the difference values by one (accessed by indexing)
    /// 
    /// # Arguments
    /// 
    /// * `i` the difference index to increment
    fn inc(&mut self, i: usize) {
        match i {
            1 => self.one += 1,
            2 => self.two += 1,
            3 => self.three += 1,
            _ => (),
        }
    }
}


fn main() {
    let lines = read_in_lines("src/adapter_ratings.txt");
    let mut adapters = Vec::new();
    adapters.push( 0 );  // Socket
    for line in lines {
        let rating = line.parse::<u32>().unwrap();
        adapters.push( rating );        
    }
    adapters.push( adapters.iter().max().unwrap() + 3 );  // Device

    let diff = sum_rating_differences(&adapters);

    println!("--- Part 1 ---\n{:#?}\n{} * {} = {}", diff, diff.one, diff.three, diff.one*diff.three);

    println!("\n--- Part 2 ---\nValid Combinations: {}", find_valid_combinations(&adapters));

    println!("\n--- General Solution ---\nValid Combinations: {}", valid_combinations_general(&adapters));
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

/// Find the differences between the valid chain of adapters.
/// 
/// # Arguments
/// 
/// * `adapters` an unsorted vector of adapters
/// 
/// # Returns
/// 
/// * A Differences struct containing the sum of 1, 2 and 3 value jumps
fn sum_rating_differences(adapters: &Vec<Adapter>) -> Differences {
    let mut diff = Differences::new();

    let mut adapters_sorted = adapters.clone();
    adapters_sorted.sort();

    let mut prev: Option<Adapter> = None;
    for adapter in adapters_sorted {
        if let Some(pre) = prev {
            diff.inc( (adapter - pre) as usize);            
        }
        prev = Some(adapter);
    }
    diff
}


/// Finds the valid number of ways of chaining the given adapters to go
/// from the smallest value to the largest.
/// 
/// Uses the fact that since all gaps are either 3 or 1 the number of
/// combinations is based solely on the length of each run of numbers.
/// Given a run of known length, the number of combinations can be found
/// from a fibbonacci like sequence where the previous three terms are
/// summed and the first three terms are 1, 1, 2.
/// 
/// This results in the values for the first few run lengths being:
/// * 1 - 1
/// * 2 - 1
/// * 3 - 2
/// * 4 - 4
/// * 5 - 7
/// * 6 - 13
/// * ...
/// 
/// By then multiplying these together you get the total number of valid
/// combinations.
/// 
/// # Arguments
/// 
/// * `adapters` an unsorted list of available adapters
/// 
/// # returns
/// 
/// * The number of valid combinations
fn find_valid_combinations(adapters: &Vec<Adapter> ) -> u64 {
    let mut result = 1;
    let mut sorted = adapters.clone();
    sorted.sort();

    let mut run = 1;
    let mut prev: Option<Adapter> = None;
    for adapter in sorted {
        if let Some(pre) = prev {
            match adapter - pre {
                1 => run += 1,
                _ => {
                    result *= fibbonacci_three(run);
                    run = 1;
                }
            }        
        }
        prev = Some(adapter);
    }
    result
}

/// Calculate the fibbonaci sequence where we sum the three
/// previous numbers. The first three terms are 1, 1, 2. 
fn fibbonacci_three(run: u64) -> u64 {
    let mut sequence = Vec::new();
    for i in 1..=run {
        match i {
            1 => sequence.insert(0, 1),
            2 => sequence.insert(0, 1),
            3 => sequence.insert(0, 2),
            _ => {
                sequence.insert(0, sequence.iter().sum());
                sequence.pop();
            }
        }
    }
    sequence[0]
}


/// General solution for finding valid combinations of any set of adapters.
/// This works no matter what gaps exist between the starting set but does
/// assume a valid solution exists.
/// 
/// # Arguments
/// 
/// * `adapters`: a vector of adapters
/// 
/// # Returns
/// 
/// * the valid number of possible combinations spanning the smallest to largest value.
fn valid_combinations_general(adapters: &Vec<Adapter>) -> u64 {
    let mut sorted = adapters.clone();
    sorted.sort();
    let mut map: Vec<AdapterMap> = Vec::new();
    // Fisrt find the valid number of connections for each adapter
    for (i, x) in sorted.iter().enumerate() {
        //Check the next three
        let mut possible_connections = 0;
        for y in sorted[(i+1)..].iter() {
            match y - x {
                d if d < 4 => possible_connections += 1,
                _ => break,
            }
        }
        let node = AdapterMap {
            adapter: *x,
            possible_connections,
        };
        map.insert(0, node);
    }

    // Now calculate total combinations
    let mut working_totals: Vec<u64> = Vec::new();
    for (i, node) in map.iter().enumerate() {
        let mut working_total = 0;
        if i == 0 {
            working_total = 1;
        } else {
            // Sum the current `working total` for the last n nodes - where n is the number
            // of valid connections the current node can make (i.e. sum the values of the
            // nodes this one can connect to )
            for connection in working_totals[..node.possible_connections as usize].iter() {
                working_total += connection;
            }
        }
        working_totals.insert(0, working_total);
    }
    working_totals[0]
}
