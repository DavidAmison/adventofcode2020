use std::collections::HashMap;

/// HashMap for storing HexTiles with their corresponding coordinate
pub type HexGrid = HashMap<HexCoordinate, HexTile>;

/// Axial coordinate in a HexGrid
pub type HexCoordinate = (i32, i32);

#[derive(Clone, Debug)]
/// Black or White colour
pub enum Colours {
    White,
    Black,
}

#[derive(Clone)]
/// Structure to store a hexagonal tiles position (using an axial coordinate system) and colour (Black or White)
pub struct HexTile {
    pub position: HexCoordinate,
    pub colour: Colours,
}

impl HexTile {
    /// Create a new tile at the given co-ordinates
    /// Coordinate position is defined as row and left diagonal
    /// i.e. from (0, 0)
    ///    ne -> (1, 0)
    ///    e  -> (0, 1)
    ///    se -> (-1, 1)
    ///    sw -> (-1, 0)
    ///    w  -> (0, -1)
    ///    nw -> (1, -1)
    pub fn new(position: HexCoordinate) -> Self {
        HexTile {
            position,
            colour: Colours::White,
        }
    }

    /// Flip the tile so the other face is up (i.e. Black -> White, White -> Black)
    pub fn flip(&mut self) {
        match self.colour {
            Colours::White => self.colour = Colours::Black,
            Colours::Black => self.colour = Colours::White,
        }
    }

    /// Interpret a series of directions around a hexagonal grid
    ///    ne -> (+1, +0)
    ///    e  -> (+0, +1)
    ///    se -> (-1, +1)
    ///    sw -> (-1, +0)
    ///    w  -> (+0, -1)
    ///    nw -> (+1 ,-1)
    /// 
    /// # Arguments
    /// 
    /// * `directions` cardinal directions with no delimiter
    /// 
    /// # Returns
    /// 
    /// * The hex coordinate - using an axial coordinate system
    pub fn find_position(directions: &String) -> HexCoordinate {
        let mut r = 0;
        let mut c = 0;       
        let mut temp: Option<char> = None;
        for ch in directions.chars() {
            if let Some(t) = temp {
                match (t, ch) {
                    ('n', 'e') => { r += 1; }
                    ('n', 'w') => { r += 1; c -= 1; }
                    ('s', 'e') => { r -= 1; c += 1; }
                    ('s', 'w') => { r -= 1; }
                    _ => panic!("Unrecognised direction {}{}", t, c),
                }
                temp = None;
            } else {
                match ch {
                    'n' => temp = Some('n'),
                    'e' => c += 1,
                    's' => temp = Some('s'),
                    'w' => c -= 1,
                    _ => panic!("Unrecognised direction {}", c),
                }
            }
        }
        (r, c)
    }
}
