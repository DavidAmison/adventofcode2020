
pub struct Edges {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub id: usize,
    pub data: Vec< Vec<usize> >,
}

impl Image {
    pub fn new() -> Self {
        Image {
            id: 0,
            data: Vec::new(),
        }
    }

    /// Print the image data row by row
    pub fn print(&self) {
        println!("\nTile {}:", self.id);
        for line in &self.data {
            for c in line {
                match c {
                    0 => print!("."),
                    1 => print!("#"),
                    _ => print!("O"),
                }
            }
            println!("");
        }
    }

    /// Get a row from the image can be a positive or negative integer
    /// If a negative integer returns rows offset from the end (i.e. -1 will return the last row)
    /// 
    /// # Arguments
    /// 
    /// * `r` the row to return
    pub fn row(&self, r: i32) -> Option< Vec<usize> > {
        if r.abs() as usize > self.data.len() {
            return None
        }
        if r >= 0 {
            Some(self.data[r as usize].clone())
        } else {
            Some(self.data[self.data.len() - r.abs() as usize].clone())
        }
    }

    /// Get a column from the image can be a positive or negative integer
    /// If a negative integer returns column offset from the end (i.e. -1 will return the last column)
    /// 
    /// # Arguments
    /// 
    /// * `c` the row to return
    pub fn col(&self, c: i32) -> Option< Vec<usize> > {
        if c.abs() as usize > self.data[0].len() {
            return None
        }
        if c >= 0 {
            Some(self.data.iter().map(|v| v[c as usize]).collect::<Vec<usize>>())
        } else {
            Some(self.data.iter().map(|v| v[v.len() - c.abs() as usize]).collect::<Vec<usize>>())
        }
    }

    /// Rotate the image through an angle (must be 90, 180 or 270 degrees)
    /// 
    /// # Arguments
    /// 
    /// * `angle` the angle to rotate through
    pub fn rotate(&mut self, angle: u32) {
        match angle {
            90 => {
                let mut new_data = Vec::new();
                for i in 0..self.data[0].len() as i32 {
                    let mut row = self.col(i).unwrap();
                    row.reverse();
                    new_data.push(row);
                }
                self.data = new_data;
            }
            180 => {
                let mut new_data = Vec::new();
                for i in 0..self.data.len() as i32 {
                    let mut row = self.row(-i-1).unwrap();
                    row.reverse();
                    new_data.push(row);
                }
                self.data = new_data;
            }
            270 => {
                let mut new_data = Vec::new();
                for i in 0..self.data[0].len() as i32 {
                    let row = self.col(-i-1).unwrap();
                    new_data.push(row);
                }
                self.data = new_data;
            }
            _ => ()
        };
    }

    /// Flips horizontally.
    ///      1 1 1      1 1 1
    /// i.e. 1 1 0  =>  0 1 1
    ///      1 0 0      0 0 1
    pub fn flip_horizontal(&mut self) {
        let mut new_data = Vec::new();
        for i in 0..self.data.len() as i32 {
            let mut new_row = self.row(i).unwrap();
            new_row.reverse();
            new_data.push(new_row);      
        }
        self.data = new_data;
    }

    /// Flips vertically (i.e. rows)
    ///       1 1 1      1 0 0
    /// i.e.  1 1 0  =>  1 1 0
    ///       1 0 0      1 0 0
    pub fn flip_vertical(&mut self) {
        let mut new_data = Vec::new();
        for i in 0..self.data.len() as i32 {
            let new_row = self.row(-i-1).unwrap();
            new_data.push(new_row);      
        }
        self.data = new_data;
    }

    /// Returns the data without the border
    pub fn trim(&self, e: Edges) -> Vec<Vec<usize>> {
        let mut picture = Vec::new();
        let max = self.data.len() - 1;
        for r in 0..max {
            if (e.top && r == 0) || (e.bottom && r == max) {
                continue;
            }
            let mut row = Vec::new();
            for c in 0..max {
                if (e.left && c == 0) || (e.right && c == max) {
                    continue;
                }
                row.push(self.data[r][c]);               
            }
            picture.push(row);
        }
        picture
    }
}

#[derive(Debug, Clone)]
pub struct ImageAssembler {
    image_array: Vec<Vec<Image>>,
    size: (usize, usize),
}

impl ImageAssembler {
    pub fn new(x: usize, y: usize) -> Self {
        ImageAssembler {
            image_array: vec!(vec!(Image::new(); y); x),
            size: (x, y),
        }    
    }

    /// Insert an image at the given x,y position
    /// Resizes the image_array as required
    pub fn insert(&mut self, i: Image, (x, y): (usize, usize)) {
        if x >= self.size.0 || y >= self.size.1 {
            panic!("Index ({},{}) out of bounds ({},{})", x, y, self.size.0, self.size.1);
        }

        // Remove borders from the image
        let copy = i.clone();
        let edges_to_trim = Edges {
            top: true,
            bottom: true,
            left: true,
            right: true,
        };
        self.image_array[x][y] = Image {
            id: copy.id,
            data: copy.trim(edges_to_trim),
        };
    }

    fn matching_edges(tile: &Image, others: &Vec<Image>) -> Edges {
        let mut matches = Edges {
            top: false,
            bottom: false,
            left: false,
            right: false,
        };
    
        let self_rows_and_cols = vec!(tile.row(0), tile.row(-1), tile.col(0), tile.col(-1));
        for other in others {
            if tile.id == other.id {
                continue;
            }
            let other_rows_and_cols = vec!(other.row(0), other.row(-1), other.col(0), other.col(-1));
            for (i, v1) in self_rows_and_cols.iter().enumerate() {
                for (_, v2) in other_rows_and_cols.iter().enumerate() {
                    let mut temp = v2.clone().unwrap();
                    temp.reverse();
                    let v2_backwards = Some(temp);
                    if v1 == v2 || v1 == &v2_backwards {
                        match i {
                            0 => matches.top = true,
                            1 => matches.bottom = true,
                            2 => matches.left = true,
                            3 => matches.right = true,
                            _ => (),
                        }                        
                    }
                }
            }
        }
        matches
    }

    fn find_next_row(x: &Image, tiles: &Vec<Image>) -> Option<Image> {
        // Column to match (last column)
        let c = x.col(-1).unwrap();
        for tile in tiles {
            if tile.id == x.id {
                continue;
            }
            let r1 = tile.row(0).unwrap();
            let rl = tile.row(-1).unwrap();
            let c1 = tile.col(0).unwrap();
            let cl = tile.col(-1).unwrap();
            let r1b = r1.iter().rev().map(|x| x.clone()).collect::<Vec<usize>>();
            let rlb = rl.iter().rev().map(|x| x.clone()).collect::<Vec<usize>>();
            let c1b = c1.iter().rev().map(|x| x.clone()).collect::<Vec<usize>>();
            let clb = cl.iter().rev().map(|x| x.clone()).collect::<Vec<usize>>();
            match c {
                _ if c == r1 => {
                    let mut t = tile.clone();
                    t.rotate(270);
                    t.flip_vertical();
                    return Some(t)
                }
                _ if c == r1b => {
                    let mut t = tile.clone();
                    t.rotate(270);
                    return Some(t)
                }
                _ if c == rl => {
                    let mut t = tile.clone();
                    t.rotate(90);
                    return Some(t)
                }
                _ if c == rlb => {
                    let mut t = tile.clone();
                    t.rotate(90);
                    t.flip_vertical();
                    return Some(t)
                }
                _ if c == c1 => {
                    let t = tile.clone();
                    return Some(t)
                }
                _ if c == c1b => {
                    let mut t = tile.clone();
                    t.flip_vertical();
                    return Some(t)
                }
                _ if c == cl => {
                    let mut t = tile.clone();
                    t.flip_horizontal();
                    return Some(t)
                }
                _ if c == clb => {
                    let mut t = tile.clone();
                    t.rotate(180);
                    return Some(t)
                }
                _ => (),
            }
        }
        None
    }

    fn find_next_column (x: &Image, tiles: &Vec<Image>) -> Option<Image> {
        // Column to match (last column)
        let r = x.row(-1).unwrap();
        for tile in tiles {
            if tile.id == x.id {
                continue;
            }
            let r1 = tile.row(0).unwrap();
            let rl = tile.row(-1).unwrap();
            let c1 = tile.col(0).unwrap();
            let cl = tile.col(-1).unwrap();
            let r1b = r1.iter().rev().map(|x| x.clone()).collect::<Vec<usize>>();
            let rlb = rl.iter().rev().map(|x| x.clone()).collect::<Vec<usize>>();
            let c1b = c1.iter().rev().map(|x| x.clone()).collect::<Vec<usize>>();
            let clb = cl.iter().rev().map(|x| x.clone()).collect::<Vec<usize>>();
            match r {
                _ if r == r1 => {
                    let t = tile.clone();
                    return Some(t)
                }
                _ if r == r1b => {
                    let mut t = tile.clone();
                    t.flip_horizontal();
                    return Some(t)
                }
                _ if r == rl => {
                    let mut t = tile.clone();
                    t.flip_vertical();
                    return Some(t)
                }
                _ if r == rlb => {
                    let mut t = tile.clone();
                    t.rotate(180);
                    return Some(t)
                }
                _ if r == c1 => {
                    let mut t = tile.clone();
                    t.rotate(90);
                    t.flip_horizontal();
                    return Some(t)
                }
                _ if r == c1b => {
                    let mut t = tile.clone();
                    t.rotate(90);
                    return Some(t)
                }
                _ if r == cl => {
                    let mut t = tile.clone();
                    t.rotate(270);
                    return Some(t)
                }
                _ if r == clb => {
                    let mut t = tile.clone();
                    t.rotate(270);
                    t.flip_horizontal();
                    return Some(t)
                }
                _ => (),
            }
        }
        None
    }

    fn collapse_image(&self) -> Image {
        let mut output_image = Vec::new();
        let mut row_offset = 0;
        let mut r = 0;
        let mut is_first = true;
        for image_row in &self.image_array {
            for image in image_row {
                for (i, row) in image.data.iter().enumerate() {
                    if is_first {
                        output_image.push(row.clone());
                        r += 1;                                             
                    } else {
                        output_image[row_offset + i].append(&mut row.clone());
                    }
                }
                is_first = false;              
            }
            row_offset += r;
            r = 0;
            is_first = true;
        }
        Image {
            id: 9999,
            data: output_image,
        } 
    }
    
    pub fn assemble(&mut self, tiles: &Vec<Image>) -> Image {
        let mut positions = Vec::new();

        // First we need to find a corner tile
        let mut start_tile: Image = Image::new();
        for tile in tiles {
            let edges = Self::matching_edges(tile, tiles);
            match (edges.top, edges.right, edges.bottom, edges.left) {
                (true, true, false, false) => { start_tile = tile.clone(); start_tile.rotate(90) }  // Top and Right
                (false, true, true, false) => { start_tile = tile.clone(); }                        // Bottom and Right
                (false, false, true, true) => { start_tile = tile.clone(); start_tile.rotate(270)}  // Bottom and Left
                (true, false, false, true) => { start_tile = tile.clone(); start_tile.rotate(180)}  // Top and Left
                _ => (),  // Not a corner
            }
        }

        // Assuming that tiles won't ever have an id of 0
        if start_tile.id == 0 {
            panic!("No corner tiels found");
        }

        // Starting with the corner tile - we find all connected tiles in the row
        // then we drop down to the next row
        let mut left_column_tile = start_tile.clone();
        let mut r = 0;
        let mut c = 0;
        loop {
            let mut current_tile = left_column_tile.clone();
            self.insert(current_tile.clone(), (r, c));
            c += 1;
            let mut row = vec!(current_tile.id);
            while let Some(next_tile) = Self::find_next_row(&current_tile, tiles) {
                row.push(next_tile.id);
                self.insert(next_tile.clone(), (r, c));
                current_tile = next_tile;
                c += 1;  // Incerement the column
            }
            positions.push(row);
            r += 1;  // Increment the row
            c = 0;
            if let Some(t) = Self::find_next_column(&left_column_tile, tiles) {
                left_column_tile = t.clone();
            } else {
                break;
            }
        }
        // for row in &positions {
        //     println!("{:?}", row);
        // }
        self.collapse_image()
    }
    
}

