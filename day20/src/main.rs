use std::fs::File;
use std::io::{BufRead, BufReader};

mod image;
use image::{Image, ImageAssembler};

fn main() {
    let tiles = read_in_tiles("src/tiles.txt");

    println!("\n--- Part 1 ---\n");
    // Copy used for finding the matching tiles
    let mut corner_product = 1;
    for t1 in &tiles {
        // Check first and last rows and columns
        if matches(t1, &tiles).len() == 2 {
            println!("{} is a corner tile", t1.id);
            corner_product *= t1.id;
        }
    }
    println!("Product of corners: {}", corner_product);


    println!("\n--- Part 2 ---");
    let mut assembler = ImageAssembler::new(12, 12);
    let mut image = assembler.assemble(&tiles);
    // Representative vector of the sea monser image
    let search_data = vec!(
        vec!(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0), 
        vec!(1,0,0,0,0,1,1,0,0,0,0,1,1,0,0,0,0,1,1,1),
        vec!(0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,0));
    let search_image = Image {
        id: 5555,
        data: search_data,
    };

    // Search throuhgh all orientations of the image
    // image.print();
    mark_image(&mut image, &search_image);
    image.rotate(90);
    mark_image(&mut image, &search_image);
    image.rotate(90);
    mark_image(&mut image, &search_image);
    image.rotate(90);
    mark_image(&mut image, &search_image);
    image.flip_vertical();
    mark_image(&mut image, &search_image);
    image.rotate(90);
    mark_image(&mut image, &search_image);
    image.rotate(90);
    mark_image(&mut image, &search_image);
    image.rotate(90);
    mark_image(&mut image, &search_image);
    image.print();
    println!("Sea Roughness = {}", count_sea_roughness(&image));
}

/// Read in lines of a file to a vector
/// 
/// # Arguments
/// 
/// * `filename` - String containing the filename
fn read_in_tiles(filename: &str) -> Vec<Image> {
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
                tiles.push( Image{id: id.unwrap(), data} );
                id = None;
                data = Vec::new();
            } else {
                data.push(line.chars().map(convert).collect::<Vec<usize>>());
            }
        }
    }
    tiles.push( Image{id: id.unwrap(), data} );
    tiles
}

fn matches(tile: &Image, others: &Vec<Image>) -> Vec<usize> {
    let mut matches = Vec::new();   
    let self_rows_and_cols = vec!(tile.row(0), tile.row(-1), tile.col(0), tile.col(-1));
    for other in others {
        if tile.id == other.id {
            continue;
        }
        let other_rows_and_cols = vec!(other.row(0), other.row(-1), other.col(0), other.col(-1));
        for (_, v1) in self_rows_and_cols.iter().enumerate() {
            for (_, v2) in other_rows_and_cols.iter().enumerate() {
                let mut temp = v2.clone().unwrap();
                temp.reverse();
                let v2_backwards = Some(temp);
                if v1 == v2 || v1 == &v2_backwards {
                    matches.push(other.id);
                }
            }
        }
    }
    matches
}

fn search(i: &Image, s: &Image) -> usize {
    let mut count = 0;
    let i_rows = i.data.len();
    let i_cols = i.data[0].len();

    let s_rows = s.data.len();
    let s_cols = s.data[0].len();

    if s_rows > i_rows || s_cols > i_cols {
        // Image is too big
        return 0
    }
    
    for r in 0..i_rows-s_rows {
        for c in 0..i_cols-s_cols {
            let mut image_found = true;
            for (x, row) in s.data.iter().enumerate() {
                for (y, pixel) in row.iter().enumerate() {
                    image_found = image_found && (pixel == &i.data[r+x][c+y] || pixel == &0);
                }
            }
            if image_found {
                count += 1;
            }
        }
    }
    count
}


fn mark_image(i: &mut Image, s: &Image) -> usize {
    let mut count = 0;
    let i_rows = i.data.len();
    let i_cols = i.data[0].len();

    let s_rows = s.data.len();
    let s_cols = s.data[0].len();

    if s_rows > i_rows || s_cols > i_cols {
        // Image is too big
        return 0
    }
    
    for r in 0..i_rows-s_rows {
        for c in 0..i_cols-s_cols {
            let mut image_found = true;
            for (x, row) in s.data.iter().enumerate() {
                for (y, pixel) in row.iter().enumerate() {
                    image_found = image_found && (pixel == &i.data[r+x][c+y] || pixel == &0);
                }
            }
            if image_found {
                count += 1;
                for (x, row) in s.data.iter().enumerate() {
                    for (y, pixel) in row.iter().enumerate() {
                        if pixel == &1 {
                            i.data[r+x][c+y] = 2;
                        }
                    }
                }
            }
        }
    }
    count
}

fn count_sea_roughness(i: &Image) -> u32 {
    let mut count = 0;
    for row in &i.data {
        for pixel in row {
            if pixel == &1 {
                count += 1;
            }
        }
    }
    count
}
