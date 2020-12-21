use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let tiles = read_in_tiles("src/tiles.txt");

    // Copy used for finding the matching tiles
    let mut corner_product = 1;
    for t1 in &tiles {
        // Check first and last rows and columns
        if t1.reassemble(&tiles) == 2 {
            println!("{} is a corner tile", t1.id);
            corner_product *= t1.id;
        }
    }
    println!("Product of corners: {}", corner_product);
}

#[derive(Clone)]
struct Tile {
    id: usize,
    data: Vec< Vec<usize> >,
}

impl Tile {
    fn print(&self) {
        println!("\nTile {}:", self.id);
        for line in &self.data {
            for c in line {
                print!("{} ", c);
            }
            println!("");
        }
    }

    fn row(&self, r: i32) -> Option< Vec<usize> > {
        if r.abs() as usize > self.data.len() {
            return None
        }
        if r >= 0 {
            Some(self.data[r as usize].clone())
        } else {
            Some(self.data[self.data.len() - r.abs() as usize].clone())
        }
    }

    fn col(&self, r: i32) -> Option< Vec<usize> > {
        if r.abs() as usize > self.data.len() {
            return None
        }
        if r >= 0 {
            Some(self.data.iter().map(|v| v[r as usize]).collect::<Vec<usize>>())
        } else {
            Some(self.data.iter().map(|v| v[v.len() - r.abs() as usize]).collect::<Vec<usize>>())
        }
    }

    fn reassemble(&self, others: &Vec<Self>) -> usize {
        let mut matches = 0;        
        let self_rows_and_cols = vec!(self.row(0), self.row(-1), self.col(0), self.col(-1));
        for other in others {
            if self.id == other.id {
                continue;
            }
            let other_rows_and_cols = vec!(other.row(0), other.row(-1), other.col(0), other.col(-1));
            for (i, v1) in self_rows_and_cols.iter().enumerate() {
                for (j, v2) in other_rows_and_cols.iter().enumerate() {
                    let mut temp = v2.clone().unwrap();
                    temp.reverse();
                    let v2_backwards = Some(temp);
                    if v1 == v2 || v1 == &v2_backwards {
                        matches += 1;
                        let x = match i {
                            0 => "Row 1",
                            1 => "Row -1",
                            2 => "Col 1",
                            3 => "Col -1",
                            _ => "NONE",  // Can't happen
                        };
                        let y = match j {
                            0 => "Row 1",
                            1 => "Row -1",
                            2 => "Col 1",
                            3 => "Col -1",
                            _ => "NONE",  // Can't happen
                        };
                        // println!("Tile {} {} matches with {} {}", self.id, x, other.id, y);
                    }
                }
            }
        }
        matches
    }

    // fn assemble(tiles: &Vec<Tile>) -> Vec<Vec<usize>> {
    //     let positions = Vec::new();

    //     // We create a mutable copy from which we will remove tiles once placed
    //     let mut copy = tiles.clone();
    //     while let tile = copy.pop() {
            
    //     }

    //     positions
    // }
}

/// Read in lines of a file to a vector
/// 
/// # Arguments
/// 
/// * `filename` - String containing the filename
fn read_in_tiles(filename: &str) -> Vec<Tile> {
    // Function to convert from charachter to usize
    let convert = |c| -> usize {
        match c {
            '#' => 1,
            '.' => 0,
            _ => panic!("Unrecognised charachter: {}", c),
        }
    };

    // Empty vector of tiles
    let mut tiles = Vec::new();

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Parse the file
    let mut id: Option<usize> = None;
    let mut data: Vec<Vec<usize>> = Vec::new();
    for line in reader.lines().flatten().collect::<Vec<String>>() {
        if id.is_none() {
            // First line must be the id of the tile (of the form "Tile [id]:")
            id = Some(line.split_whitespace().nth(1).unwrap().trim_matches(':').parse::<usize>().unwrap());
        } else {
            if line.trim().is_empty() {
                tiles.push( Tile{id: id.unwrap(), data} );
                id = None;
                data = Vec::new();
            } else {
                data.push(line.chars().map(convert).collect::<Vec<usize>>());
            }
        }
    }
    tiles.push( Tile{id: id.unwrap(), data} );
    tiles
}
