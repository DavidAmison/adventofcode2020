use std::collections::HashMap;


pub type HyperSpace = HashMap<Coordinate, u32>;
pub type Coordinate = (i32, i32, i32, i32);

impl HyperCube {
    pub fn new() -> Self {
        HashMap::new()
    }

    pub fn point_exists(&self, point: Coordinate) -> bool {
        self.contains_key(&point)        
    }

    /// Insert blank (0 value) cells all around the given coordinate (if the cell doesn't exist)
    /// 
    /// # Arguments
    /// 
    /// * `
    pub fn insert_around_point(&self, point: Coordinate) {
        for w in point.0-1..=point.0+1 {
            for z in point.1-1..=point.1+1 {
                for y in point.2-1..=point.2+1 {
                    for x in point.3-1..=point.3+1 {
                        let p: Coordinate = (w, x, y, z);
                        if !self.point_exists( p ) {
                            self.insert(p, 0);
                        }
                    }
                }
            }
        }
    }

    pub fn insert(&self, point: Coordinate, value: i32) -> Option(i32) {
        Err()
    }
}