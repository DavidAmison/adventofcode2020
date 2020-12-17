mod hyperspace;
use hyperspace::*;

mod space;
use space::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_one();
    part_two();

}

fn part_one() {
    println!("\n--- Part 1 ---");
    // Read in plane
    let initial_setup = read_in_file_to_matrix("src/initial_state.txt");
    // Convert plane to 3dSpace
    let mut space = Space::new();
    for (j, y) in initial_setup.iter().enumerate() {
        for (i, x) in y.iter().enumerate() {
            let p: Coordinate3d = (i as i32, j as i32, 0);
            match x {
                '#' => {
                    space.insert(p, 1);
                    space.insert_around_point(p);
                }
                _ => (),
            };
        }
    }

    for _ in 0..6 {
        let mut next_space = Space::new();
        for (p, v) in space.points() {
            let s = space.sum_around_point(*p);
            match v {
                0 => {
                    if s == 3 {
                        next_space.insert(*p, 1);
                        next_space.insert_around_point(*p);
                    }
                }
                1 => {
                    if s == 2 || s == 3 {
                        next_space.insert(*p, 1);
                        next_space.insert_around_point(*p);
                    }
                }
                _ => (),
            };
        }
        space = next_space;
    }

    println!("Sum of active cells: {}", space.sum_all_points());  
}

fn part_two() {
    println!("\n--- Part 2 ---");
    // Read in plane
    let initial_setup = read_in_file_to_matrix("src/initial_state.txt");
    // Convert plane to 3dSpace
    let mut space = HyperSpace::new();
    for (j, y) in initial_setup.iter().enumerate() {
        for (i, x) in y.iter().enumerate() {
            let p: Coordinate4d = (0, i as i32, j as i32, 0);
            match x {
                '#' => {
                    space.insert(p, 1);
                    space.insert_around_point(p);
                }
                _ => (),
            };
        }
    }

    for _ in 0..6 {
        let mut next_space = HyperSpace::new();
        for (p, v) in space.points() {
            let s = space.sum_around_point(*p);
            match v {
                0 => {
                    if s == 3 {
                        next_space.insert(*p, 1);
                        next_space.insert_around_point(*p);
                    }
                }
                1 => {
                    if s == 2 || s == 3 {
                        next_space.insert(*p, 1);
                        next_space.insert_around_point(*p);
                    }
                }
                _ => (),
            };
        }
        space = next_space;
    }

    println!("Sum of active cells: {}", space.sum_all_points()); 
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
