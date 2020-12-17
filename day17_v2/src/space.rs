use std::collections::HashMap;


pub type Coordinate3d = (i32, i32, i32);

#[derive(Debug)]
pub struct Space { space: HashMap<Coordinate3d, i32> }

impl Space {
    pub fn new() -> Self {
        Space{ space: HashMap::new() }
    }

    pub fn point_exists(&self, point: Coordinate3d) -> bool {
        self.space.contains_key(&point)        
    }

    /// Insert blank (0 value) cells all around the given coordinate3d (if the cell doesn't exist)
    /// 
    /// # Arguments
    /// 
    /// * `
    pub fn insert_around_point(&mut self, point: Coordinate3d) {
        for z in point.2-1..=point.2+1 {
            for y in point.1-1..=point.1+1 {
                for x in point.0-1..=point.0+1 {
                    let p: Coordinate3d = (x, y, z);
                    if p != point && !self.point_exists( p ) {
                        self.space.insert(p, 0);
                    }
                }
            }
        }
    }

    pub fn insert(&mut self, point: Coordinate3d, value: i32) -> Option<i32> {
        self.space.insert(point, value)
    }

    pub fn get_value(&self, point: Coordinate3d) -> Option<&i32> {
        self.space.get(&point)
    }

    pub fn sum_around_point(&self, point: Coordinate3d) -> i32 {
        let mut total = 0;
        for z in point.2-1..=point.2+1 {
            for y in point.1-1..=point.1+1 {
                for x in point.0-1..=point.0+1 {
                    let p: Coordinate3d = (x, y, z);
                    if p != point {
                        if let Some(v) = self.get_value(p) {
                            total += v;                                                        
                        }
                    }
                }
            }
        }
        total
    }

    pub fn print(&self) {
        for (p, v) in self.space.iter() {
            println!("({}, {}, {}) => {}", p.0, p.1, p.2, v);
        }
    }

    pub fn points(&self) -> &HashMap<Coordinate3d, i32> {
        &self.space
    }

    pub fn sum_all_points(&self) -> i32 {
        let mut total = 0;
        for (_, v) in self.space.iter() {
            total += v;
        }
        total
    }
}