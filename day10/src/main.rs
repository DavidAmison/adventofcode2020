/**
 * --- Day 10: Adapter Array ---
 * 
 * Patched into the aircraft's data port, you discover weather forecasts of a massive tropical storm. Before you can figure out whether it will impact your vacation plans, however, your device suddenly turns off!
 * Its battery is dead.
 * You'll need to plug it in. There's only one problem: the charging outlet near your seat produces the wrong number of jolts. Always prepared, you make a list of all of the joltage adapters in your bag.
 * Each of your joltage adapters is rated for a specific output joltage (your puzzle input). Any given adapter can take an input 1, 2, or 3 jolts lower than its rating and still produce its rated output joltage.
 * In addition, your device has a built-in joltage adapter rated for 3 jolts higher than the highest-rated adapter in your bag. (If your adapter list were 3, 9, and 6, your device's built-in adapter would be rated for 12 jolts.)
 * Treat the charging outlet near your seat as having an effective joltage rating of 0.
 * Since you have some time to kill, you might as well test all of your adapters. Wouldn't want to get to your resort and realize you can't even charge your device!
 * If you use every adapter in your bag at once, what is the distribution of joltage differences between the charging outlet, the adapters, and your device?
 * For example, suppose that in your bag, you have adapters with the following joltage ratings:
 * 
 * 16
 * 10
 * 15
 * 5
 * 1
 * 11
 * 7
 * 19
 * 6
 * 12
 * 4
 * 
 * With these adapters, your device's built-in joltage adapter would be rated for 19 + 3 = 22 jolts, 3 higher than the highest-rated adapter.
 * Because adapters can only connect to a source 1-3 jolts lower than its rating, in order to use every adapter, you'd need to choose them like this:
 * 
 *     The charging outlet has an effective rating of 0 jolts, so the only adapters that could connect to it directly would need to have a joltage rating of 1, 2, or 3 jolts. Of these, only one you have is an adapter rated 1 jolt (difference of 1).
 *     From your 1-jolt rated adapter, the only choice is your 4-jolt rated adapter (difference of 3).
 *     From the 4-jolt rated adapter, the adapters rated 5, 6, or 7 are valid choices. However, in order to not skip any adapters, you have to pick the adapter rated 5 jolts (difference of 1).
 *     Similarly, the next choices would need to be the adapter rated 6 and then the adapter rated 7 (with difference of 1 and 1).
 *     The only adapter that works with the 7-jolt rated adapter is the one rated 10 jolts (difference of 3).
 *     From 10, the choices are 11 or 12; choose 11 (difference of 1) and then 12 (difference of 1).
 *     After 12, only valid adapter has a rating of 15 (difference of 3), then 16 (difference of 1), then 19 (difference of 3).
 *     Finally, your device's built-in adapter is always 3 higher than the highest adapter, so its rating is 22 jolts (always a difference of 3).
 * 
 * In this example, when using every adapter, there are 7 differences of 1 jolt and 5 differences of 3 jolts.
 * Here is a larger example:
 * 
 * 28
 * 33
 * 18
 * 42
 * 31
 * 14
 * 46
 * 20
 * 48
 * 47
 * 24
 * 23
 * 49
 * 45
 * 19
 * 38
 * 39
 * 11
 * 1
 * 32
 * 25
 * 35
 * 8
 * 17
 * 7
 * 9
 * 4
 * 2
 * 34
 * 10
 * 3
 * 
 * In this larger example, in a chain that uses all of the adapters, there are 22 differences of 1 jolt and 10 differences of 3 jolts.
 * Find a chain that uses all of your adapters to connect the charging outlet to your device's built-in adapter and count the joltage differences between the charging outlet, the adapters, and your device. What is the number of 1-jolt differences multiplied by the number of 3-jolt differences?
 * 
 * --- Part Two ---
 * 
 * To completely determine whether you have enough adapters, you'll need to figure out how many different ways they can be arranged. Every arrangement needs to connect the charging outlet to your device. The previous rules about when adapters can successfully connect still apply.
 * The first example above (the one that starts with 16, 10, 15) supports the following arrangements:
 * 
 * (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
 * (0), 1, 4, 5, 6, 7, 10, 12, 15, 16, 19, (22)
 * (0), 1, 4, 5, 7, 10, 11, 12, 15, 16, 19, (22)
 * (0), 1, 4, 5, 7, 10, 12, 15, 16, 19, (22)
 * (0), 1, 4, 6, 7, 10, 11, 12, 15, 16, 19, (22)
 * (0), 1, 4, 6, 7, 10, 12, 15, 16, 19, (22)
 * (0), 1, 4, 7, 10, 11, 12, 15, 16, 19, (22)
 * (0), 1, 4, 7, 10, 12, 15, 16, 19, (22)
 * 
 * (The charging outlet and your device's built-in adapter are shown in parentheses.) Given the adapters from the first example, the total number of arrangements that connect the charging outlet to your device is 8.
 * The second example above (the one that starts with 28, 33, 18) has many arrangements. Here are a few:
 * 
 * (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
 * 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, (52)
 * 
 * (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
 * 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 49, (52)
 * 
 * (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
 * 32, 33, 34, 35, 38, 39, 42, 45, 46, 48, 49, (52)
 * 
 * (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
 * 32, 33, 34, 35, 38, 39, 42, 45, 46, 49, (52)
 * 
 * (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
 * 32, 33, 34, 35, 38, 39, 42, 45, 47, 48, 49, (52)
 * 
 * (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
 * 46, 48, 49, (52)
 * 
 * (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
 * 46, 49, (52)
 * 
 * (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
 * 47, 48, 49, (52)
 * 
 * (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
 * 47, 49, (52)
 * 
 * (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
 * 48, 49, (52)
 * 
 * In total, this set of adapters can connect the charging outlet to your device in 19208 distinct arrangements.
 * You glance back down at your bag and try to remember why you brought so many adapters; there must be more than a trillion valid ways to arrange them! Surely, there must be an efficient way to count the arrangements.
 * 
 * What is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device?
 */

use std::fs::File;
use std::io::{BufRead, BufReader};

type Adapter = u32;

/// A structure to hold the adapters along with how many others it is able to connect to.
#[derive(Debug)]
struct AdapterMap {
    adapter: Adapter,
    possible_connections: u64,
}

/// A structure for storing the differences between the chain of adapters.
/// The maximum difference considered is three and minimum is one
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

    println!("\n--- Part 2 ---\nValid Combinations: {}", valid_combinations(&adapters));
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
fn valid_combinations(adapters: &Vec<Adapter>) -> u64 {
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
