/**
 * --- Day 11: Seating System ---
 * Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!
 * By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).
 * 
 * The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:
 * L.LL.LL.LL
 * LLLLLLL.LL
 * 
 * L.L.L..L..
 * LLLL.LL.LL
 * L.LL.LL.LL
 * L.LLLLL.LL
 * ..L.L.....
 * LLLLLLLLLL
 * L.LLLLLL.L
 * L.LLLLL.LL
 * Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the number of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:
 * 
 * If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
 * 
 * If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
 * Otherwise, the seat's state does not change.
 * Floor (.) never changes; seats don't move, and nobody sits on the floor.
 * After one round of these rules, every seat in the example layout becomes occupied:
 *  
 * #.##.##.##
 * #######.##
 * #.#.#..#..
 * ####.##.##
 * #.##.##.##
 * #.#####.##
 * ..#.#.....
 * ##########
 * #.######.#
 * #.#####.##
 * After a second round, the seats with four or more occupied adjacent seats become empty again:
 * 
 * #.LL.L#.##
 * #LLLLLL.L#
 * L.L.L..L..
 * #LLL.LL.L#
 * #.LL.LL.LL
 * #.LLLL#.##
 * ..L.L.....
 * #LLLLLLLL#
 * #.LLLLLL.L
 * #.#LLLL.##
 * #.##.L#.##
 * #L###LL.L#
 * 
 * This process continues for three more rounds:
 * 
 * L.#.#..#..
 * #L##.##.L#
 * #.##.LL.LL
 * #.###L#.##
 * ..#.#.....
 * #L######L#
 * #.LL###L.L
 * #.#L###.##
 * #.#L.L#.##
 * #LLL#LL.L#
 * 
 * L.L.L..#..
 * #LLL.##.L#
 * #.LL.LL.LL
 * #.LL#L#.##
 * ..L.L.....
 * #L#LLLL#L#
 * #.LLLLLL.L
 * #.#L#L#.##
 * 
 * #.#L.L#.##
 * #LLL#LL.L#
 * L.#.L..#..
 * #L##.##.L#
 * #.#L.LL.LL
 * #.#L#L#.##
 * ..L.L.....
 * #L#L##L#L#
 * #.LLLLLL.L
 * #.#L#L#.##
 * 
 * At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.
 * Simulate your seating area by applying the seating rules repeatedly until no seats change state. How many seats end up occupied?
 * 
 * --- Part Two ---
 * 
 * As soon as people start to arrive, you realize your mistake. People don't just care about adjacent seats - they care about the first seat they can see in each of those eight directions!
 * Now, instead of considering just the eight immediately adjacent seats, consider the first seat in each of those eight directions. For example, the empty seat below would see eight occupied seats:
 * 
 * .......#.
 * ...#.....
 * .#.......
 * .........
 * ..#L....#
 * ....#....
 * .........
 * #........
 * ...#.....
 * 
 * The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied ones:
 * 
 * .............
 * .L.L.#.#.#.#.
 * .............
 * 
 * The empty seat below would see no occupied seats:
 * 
 * .##.##.
 * #.#.#.#
 * ##...##
 * ...L...
 * ##...##
 * #.#.#.#
 * .##.##.
 * 
 * Also, people seem to be more tolerant than you expected: it now takes five or more visible occupied seats for an occupied seat to become empty (rather than four or more from the previous rules). The other rules still apply: empty seats that see no occupied seats become occupied, seats matching no rule don't change, and floor never changes.
 * Given the same starting layout as above, these new rules cause the seating area to shift around as follows:
 * 
 * L.LL.LL.LL
 * LLLLLLL.LL
 * L.L.L..L..
 * LLLL.LL.LL
 * L.LL.LL.LL
 * L.LLLLL.LL
 * ..L.L.....
 * LLLLLLLLLL
 * L.LLLLLL.L
 * L.LLLLL.LL
 * 
 * #.##.##.##
 * #######.##
 * #.#.#..#..
 * ####.##.##
 * #.##.##.##
 * #.#####.##
 * ..#.#.....
 * ##########
 * #.######.#
 * #.#####.##
 * 
 * #.LL.LL.L#
 * #LLLLLL.LL
 * L.L.L..L..
 * LLLL.LL.LL
 * L.LL.LL.LL
 * L.LLLLL.LL
 * ..L.L.....
 * LLLLLLLLL#
 * #.LLLLLL.L
 * #.LLLLL.L#
 * 
 * #.L#.##.L#
 * #L#####.LL
 * L.#.#..#..
 * ##L#.##.##
 * #.##.#L.##
 * #.#####.#L
 * ..#.#.....
 * LLL####LL#
 * #.L#####.L
 * #.L####.L#
 * 
 * #.L#.L#.L#
 * #LLLLLL.LL
 * L.L.L..#..
 * ##LL.LL.L#
 * L.LL.LL.L#
 * #.LLLLL.LL
 * ..L.L.....
 * LLLLLLLLL#
 * #.LLLLL#.L
 * #.L#LL#.L#
 * 
 * #.L#.L#.L#
 * #LLLLLL.LL
 * L.L.L..#..
 * ##L#.#L.L#
 * L.L#.#L.L#
 * #.L####.LL
 * ..#.#.....
 * LLL###LLL#
 * #.LLLLL#.L
 * #.L#LL#.L#
 * 
 * #.L#.L#.L#
 * #LLLLLL.LL
 * L.L.L..#..
 * ##L#.#L.L#
 * L.L#.LL.L#
 * #.LLLL#.LL
 * ..#.L.....
 * LLL###LLL#
 * #.LLLLL#.L
 * #.L#LL#.L#
 * 
 * Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once this occurs, you count 26 occupied seats.
 * Given the new visibility method and the rule change for occupied seats becoming empty, once equilibrium is reached, how many seats end up occupied?
 */

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_one();
    part_two();
}


/// Find the solution for part 1 - finding the stable seating layout for the given input and rules.
fn part_one() {
    let initial_layout = read_in_file_to_matrix("src/initial_layout.txt");
    let rows = initial_layout.len();
    let columns = initial_layout[0].len();
    let mut is_changed = true;
    let mut current = initial_layout.clone();
    while is_changed {
        is_changed = false;
        let mut next: Vec< Vec<char> > = Vec::new();
        for r in 0..rows {
            let mut row: Vec<char> = Vec::new();
            for c in 0..columns {
                let occupied_adjacent = count_occupied_adjacent(r as i32, c as i32, &current);
                match current[r][c] {
                    'L' => {
                        if occupied_adjacent == 0 {
                            row.push('#');
                            is_changed = true;
                        } else {
                            row.push('L');
                        }
                    },
                    '#' => {
                        if occupied_adjacent >= 4 {
                            row.push('L');
                            is_changed = true;
                        } else {
                            row.push('#');
                        }
                    },
                    '.' => row.push('.'),
                    _ => (),
                }
            }
            next.push(row);
        }
        current = next;
        // for row in &current {
        //     println!("{:?}", row)
        // }
        // println!("\n\n");
    }
    println!("\n--- Part 1 ---\n");
    println!("Total Occupied = {}", count_total_occupied(&current));   
}


/// Find the solution for part 2 - finding the stable seating layout for the given input using the LOS rule.
fn part_two() {
    let initial_layout = read_in_file_to_matrix("src/initial_layout.txt");
    let rows = initial_layout.len();
    let columns = initial_layout[0].len();
    let mut is_changed = true;
    let mut current = initial_layout.clone();
    while is_changed {
        is_changed = false;
        let mut next: Vec< Vec<char> > = Vec::new();
        for r in 0..rows {
            let mut row: Vec<char> = Vec::new();
            for c in 0..columns {
                let occupied_adjacent = count_occupied_line_of_sight(r as i32, c as i32, &current);
                match current[r][c] {
                    'L' => {
                        if occupied_adjacent == 0 {
                            row.push('#');
                            is_changed = true;
                        } else {
                            row.push('L');
                        }
                    },
                    '#' => {
                        if occupied_adjacent >= 5 {
                            row.push('L');
                            is_changed = true;
                        } else {
                            row.push('#');
                        }
                    },
                    '.' => row.push('.'),
                    _ => (),
                }
            }
            next.push(row);
        }
        current = next;
    }
    println!("\n--- Part 2 ---\n");
    // for row in &current {
    //     println!("{:?}", row)
    // }
    println!("Total Occupied = {}", count_total_occupied(&current));   
}


/// Read in lines of a file to a vector
/// 
/// # Arguments
/// 
/// * `filename` - String containing the filename
fn read_in_file_to_matrix(filename: &str) -> Vec<Vec<char>> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|x| -> Vec<char> { x.unwrap().chars().collect() } ).collect()
}


/// Counts all occupied seats (#) directly adjacent to the given seat
/// 
/// # Arguments
/// 
/// * `r` the row of the seat to check for
/// * `c` the column of the seat to check for
/// * `seating` state of the seating (charachter vector: # = occupied, L = empty, . = floor)
/// 
/// # Returns
/// 
/// * The number of occupied seats (#) adjacent to the one given
fn count_occupied_adjacent(r: i32, c: i32, seating: &Vec< Vec<char> >) -> u32 {
    let rows = seating.len() as i32;
    let cols = seating[0].len() as i32;
    
    let mut count = 0;
    for i in r-1..=r+1 {
        for j in c-1..=c+1 {
            if i == r && j == c {
                continue
            }
            match (i, j) {
                (i, j) if i >= 0 && i < rows && j >= 0 && j < cols => {
                    match seating[i as usize][j as usize] {
                        '#' => count += 1,
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }
    count
}

/// Counts all occupied seats (#) directly in line of sight of the given seat
/// 
/// # Arguments
/// 
/// * `r` the row of the seat to check for
/// * `c` the column of the seat to check for
/// * `seating` state of the seating (charachter vector: # = occupied, L = empty, . = floor)
/// 
/// # Returns
/// 
/// * The number of occupied seats (#) adjacent to the one given
fn count_occupied_line_of_sight(r: i32, c: i32, seating: &Vec< Vec<char> >) -> u32 {
    let rows = seating.len() as i32;
    let cols = seating[0].len() as i32;

    fn seat_exists(r: i32, c: i32, max_rows: i32, max_cols: i32) -> bool {
        r >= 0 && r < max_rows && c >= 0 && c < max_cols
    }
    
    let mut count = 0;
    for dir_r in &[1, 0, -1] {
        for dir_c in &[1, 0, -1] {
            if *dir_r == 0 && *dir_c == 0 {
                continue;
            }
            let mut x = r + dir_r;
            let mut y = c + dir_c;
            while seat_exists(x, y, rows, cols) {
                match seating[x as usize][y as usize] {
                    '#' => {
                        count += 1;
                        break;
                    },
                    'L' => break,
                    _ => (),
                }
                x += dir_r;
                y += dir_c;
            }
        }
    }
    count
}


/// Count the total number of occupied seats
/// 
/// # Arguments
/// 
/// * `seating` the current seating layout (charachter vector: # = occupied, L = empty, . = floor)
/// 
/// # Returns
/// 
/// * the number of occupied seats (represented by #)
fn count_total_occupied(seating: &Vec< Vec<char> >) -> u64 {
    let mut count = 0;
    for row in seating {
        for seat in row {
            match seat {
                '#' => count += 1,
                _ => (),
            }
        }
    }
    count
}