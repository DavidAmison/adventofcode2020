use std::fs::File;
use std::io::{BufRead, BufReader};

type Cube = Vec<Vec<Vec<char>>>;
type HyperCube = Vec<Vec<Vec<Vec<char>>>>;

type XYZ = (i32, i32, i32);
type WXYZ = (i32, i32, i32, i32);


fn main() {
    part_one();
    part_two();
}


/// Find the solution for part 1
fn part_one() {
    println!("\n--- Part 1 ---");
    // Read in plane and make into a cube
    let initial_layout = vec!(read_in_file_to_matrix("src/initial_state.txt"));

    let mut lz = initial_layout.len();
    let mut ly = initial_layout[0].len();
    let mut lx = initial_layout[0][0].len();
    let mut current = initial_layout.clone();  

    for _ in 0..6 {
        // Resize the matrix    
        current.push(vec!(vec!('.'; lx); ly));
        current.insert(0, vec!(vec!('.'; lx); ly));
        for z in 0..lz+2 {
            current[z].push(vec!('.'; lx));
            current[z].insert(0, vec!('.'; lx));
            for y in 0..ly+2 {
                current[z][y].push('.');
                current[z][y].insert(0, '.');
            }
        }
        lx += 2;
        ly += 2;
        lz += 2;
        
        // Find the next state of the Cube
        let mut next: Cube = Vec::new();
        for z in 0..lz {
            let mut plane = Vec::new();
            for y in 0..ly {
                let mut line = Vec::new();
                for x in 0..lx {
                    let active = count_active_cubes_adjacent_3d((x as i32, y as i32, z as i32), &current);
                    match current[z][y][x] {
                        '.' => {
                            if active == 3 {
                                line.push('#');                                
                            } else {
                                line.push('.');
                            }
                        }
                        '#' => {
                            if active == 2 || active == 3 {
                                line.push('#');
                            } else {
                                line.push('.');
                            }
                        }
                        c => panic!("Unrecognised character: {}", c), 
                    }
                }
                plane.push(line);
            }
            next.push(plane);
        }
        current = next;
    }
    println!("Active cubes: {}", count_active_total_3d(&current));
}

/// Find the solution for part 2
fn part_two() {
    println!("\n--- Part 2 ---");
    // Read in plane and make into a 4d matrix
    let initial_layout = vec!(vec!(read_in_file_to_matrix("src/initial_state.txt")));

    let mut lw = initial_layout.len();
    let mut lz = initial_layout[0].len();
    let mut ly = initial_layout[0][0].len();
    let mut lx = initial_layout[0][0][0].len();
    let mut current = initial_layout.clone();
    
    for _ in 0..6 {
        // Resize the matrix    
        current.push(vec!(vec!(vec!('.'; lx); ly); lz));
        current.insert(0, vec!(vec!(vec!('.'; lx); ly); lz));
        for w in 0..lw+2 {
            current[w].push(vec!(vec!('.'; lx); ly));
            current[w].insert(0, vec!(vec!('.'; lx); ly));
            for z in 0..lz+2 {
                current[w][z].push(vec!('.'; lx));
                current[w][z].insert(0, vec!('.'; lx));
                for y in 0..ly+2 {
                    current[w][z][y].push('.');
                    current[w][z][y].insert(0, '.');
                }
            }
        }
        lx += 2;
        ly += 2;
        lz += 2;
        lw += 2;
    
        // Find the next state of the HyperCube
        let mut next: HyperCube = Vec::new();
        for w in 0..lw {
            let mut cube = Vec::new();
            for z in 0..lz {
                let mut plane = Vec::new();
                for y in 0..ly {
                    let mut line = Vec::new();
                    for x in 0..lx {
                        let active = count_active_cubes_adjacent_4d((x as i32, y as i32, z as i32, w as i32), &current);
                        match current[w][z][y][x] {
                            '.' => {
                                if active == 3 {
                                    line.push('#');                                
                                } else {
                                    line.push('.');
                                }
                            }
                            '#' => {
                                if active == 2 || active == 3 {
                                    line.push('#');
                                } else {
                                    line.push('.');
                                }
                            }
                            c => panic!("Unrecognised character: {}", c), 
                        }
                    }
                    plane.push(line);
                }
                cube.push(plane);
            }
            next.push(cube);
        }
        current = next;
    }
    println!("Active cubes: {}", count_active_total_4d(&current));
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


/// Counts all active cubes (#) directly adjacent to the given co-ordinate
/// 
/// # Arguments
/// 
/// * `(x, y, z)` the coordinate of the cube to check
/// * `pocket_dimension` state of the pocket dimension (charachter vector: # = active, . = inactive)
/// 
/// # Returns
/// 
/// * The number of active cubes (#) adjacent to the one given
fn count_active_cubes_adjacent_3d((x, y, z): XYZ, pocket_dimension: &Cube) -> u32 {
    let lz = pocket_dimension.len() as i32;
    let ly = pocket_dimension[0].len() as i32;
    let lx = pocket_dimension[0][0].len() as i32;
    
    let mut count = 0;
    for k in z-1..=z+1 {
        for j in y-1..=y+1 {
            for i in x-1..=x+1 {
                if k==z && j==y && i==x {
                    continue;
                }
                match (i, j, k) {
                    (i, j, k) if i >= 0 && i < lx && j >= 0 && j < ly && k >= 0 && k < lz => {
                        match pocket_dimension[k as usize][j as usize][i as usize] {
                            '#' => count += 1,
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    count
}

/// Count the number of active cubes in the cube space
/// 
/// # Arguments
fn count_active_total_3d(pocket_dimension: &Cube) -> u32 {
    let mut active_count = 0;
    for z in pocket_dimension {
        for y in z {
            for x in y {
                if *x == '#' {
                    active_count += 1;
                }
            }
        }
    }
    active_count
}

/// Counts all active cubes (#) directly adjacent to the given co-ordinate
/// 
/// # Arguments
/// 
/// * `(x, y, z, w)` the coordinate of the cube to check
/// * `pocket_dimension` state of the pocket dimension (charachter vector: # = active, . = inactive)
/// 
/// # Returns
/// 
/// * The number of active cubes (#) adjacent to the one given
fn count_active_cubes_adjacent_4d((x, y, z, w): WXYZ, pocket_dimension: &HyperCube) -> u32 {
    let lw = pocket_dimension.len() as i32;
    let lz = pocket_dimension[0].len() as i32;
    let ly = pocket_dimension[0][0].len() as i32;
    let lx = pocket_dimension[0][0][0].len() as i32;
    
    let mut count = 0;
    for l in w-1..=w+1 {
        for k in z-1..=z+1 {
            for j in y-1..=y+1 {
                for i in x-1..=x+1 {
                    if l==w && k==z && j==y && i==x {
                        continue;
                    }
                    match (i, j, k, l) {
                        (i, j, k, l) if i >= 0 && i < lx && j >= 0 && j < ly && k >= 0 && k < lz && l >=0 && l < lw => {
                            match pocket_dimension[l as usize][k as usize][j as usize][i as usize] {
                                '#' => count += 1,
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    count
}

/// Count the number of active cubes in the hypercube space
/// 
/// # Arguments
fn count_active_total_4d(pocket_dimension: &HyperCube) -> u32 {
    let mut active_count = 0;
    for w in pocket_dimension {
        for z in w {
            for y in z {
                for x in y {
                    if *x == '#' {
                        active_count += 1;
                    }
                }
            }
        }
    }
    active_count
}
