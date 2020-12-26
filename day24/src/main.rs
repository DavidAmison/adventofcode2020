use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

mod hexgrid;
use hexgrid::{HexTile, Colours, HexGrid, HexCoordinate};

fn main() {
    // Read in the tiles and interpret the directions for placing/flipping tiles
    let lines = read_in_lines("src/positions.txt");
    let mut tiles: HexGrid = HashMap::new();
    for line in &lines {
        let position = HexTile::find_position(&line);
        if let Some(tile) = tiles.get_mut(&position) {
            // If tile already exists then flip it
            tile.flip();
        } else {
            // Otherwise create the tile and flip it (so that it is black)
            let mut tile = HexTile::new(position);
            tile.flip();
            tiles.insert(position, tile);
        }
    }
    
    // Create grid with only black tiles and their neighbours (since that is all we care about on each iteration)
    let mut next: HexGrid = HashMap::new();
    for tile in tiles.values() {
        match tile.colour {
            Colours::Black => {
                next.insert(tile.position, tile.clone());
                insert_around_tile(tile.position, &mut next);
            }
            _ => (),
        }
    }
    tiles = next;

    println!("\n--- Part 1 ---");
    println!("Number of black tiles: {}", count_black_tiles(&tiles));

    println!("\n--- Part 2 ---");
    // Iterate 100 times according to the rules:
    // *  Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
    // *  Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
    for _ in 0..100 {
        let mut next: HexGrid = HashMap::new();
        for tile in tiles.values() {
            let count = count_black_around_tile(tile.position, &tiles);
            let p = tile.position;
            // We are only inserting any tiles that qualify as black (and their immediate neighbours)
            match (tile.colour.clone(), count) {
                (Colours::White, 2) => {
                    // Flip white tiles surrounded by exactly 2 black tiles
                    let mut t = HexTile::new(p);
                    t.flip();
                    next.insert(p, t);
                    insert_around_tile(p, &mut next);
                }
                (Colours::Black, _) if count > 0 && count < 3 => {
                    // This tile stays black
                    let mut t = HexTile::new(p);
                    t.flip();
                    next.insert(p, t);
                    insert_around_tile(p, &mut next);
                }
                (_, _) => (),  // Do nothing - the tile would be white
            }
        }
        tiles = next;
    }
    println!("Day 100: {}", count_black_tiles(&tiles));
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

/// Count the number of black tiles around a given HexTile
/// 
/// # Arguments
/// 
/// * `p` the point to look around
/// * `tiles` HexGrid of tiles to check 
fn count_black_around_tile(p: HexCoordinate, tiles: &HexGrid) -> u32 {
    // Given position adjacent tiles are:
    let adjacent = vec!((p.0  , p.1+1),     // e : (+0, +1)
                        (p.0-1, p.1+1),     // se: (-1, +1)
                        (p.0-1, p.1  ),     // sw: (-1, +0)
                        (p.0  , p.1-1),     // w : (+0, -1)
                        (p.0+1, p.1-1),     // nw: (+1, -1)
                        (p.0+1, p.1  ));    // ne: (+1, +0)
    let mut count = 0;
    for point in adjacent {
        if let Some(tile) = tiles.get(&point) {
            match tile.colour {
                Colours::White => (),
                Colours::Black => {
                    count += 1;
                }
            }
        }
    }
    count
}

/// Insert white tiles around the given hex coordinate
/// 
/// # Arguments
/// 
/// * `p` the HexCoordinate to insert the tiles around
/// * `tiles` the HexGrid to modify
fn insert_around_tile(p: HexCoordinate, tiles: &mut HexGrid) {
    // Given position adjacent tiles are:
    let adjacent = vec!((p.0  , p.1+1),     // e : (+0, +1)
                        (p.0-1, p.1+1),     // se: (-1, +1)
                        (p.0-1, p.1  ),     // sw: (-1, +0)
                        (p.0  , p.1-1),     // w : (+0, -1)
                        (p.0+1, p.1-1),     // nw: (+1, -1)
                        (p.0+1, p.1  ));    // ne: (+1, +0)

    for point in adjacent {
        if !tiles.contains_key(&point) {
            tiles.insert(point, HexTile::new(point));
        }
    }
}

/// Counts the total number of black tiles in a HexGrid
/// 
/// # Arguments
/// 
/// * `tiles` a HexGrid
/// 
/// # Returns
/// 
/// * the number of black tiles as u32
fn count_black_tiles(tiles: &HexGrid) -> u32 {
    let mut black_tiles = 0;
    for tile in tiles.values() {
        match tile.colour {
            Colours::White => (),
            Colours::Black => {
                black_tiles += 1;
            }
        }
    }
    black_tiles
}
