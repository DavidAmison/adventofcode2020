use std::collections::HashMap;


pub type Coordinate4d = (i32, i32, i32, i32);

#[derive(Debug)]
pub struct HyperSpace { space: HashMap<Coordinate4d, i32> }

impl HyperSpace {
    pub fn new() -> Self {
        HyperSpace{ space: HashMap::new() }
    }

    pub fn point_exists(&self, point: Coordinate4d) -> bool {
        self.space.contains_key(&point)        
    }

    /// Insert blank (0 value) cells all around the given Coordinate4d (if the cell doesn't exist)
    /// 
    /// # Arguments
    /// 
    /// * `
    pub fn insert_around_point(&mut self, point: Coordinate4d) {
        for w in point.0-1..=point.0+1 {
            for z in point.3-1..=point.3+1 {
                for y in point.2-1..=point.2+1 {
                    for x in point.1-1..=point.1+1 {
                        let p: Coordinate4d = (w, x, y, z);
                        if p != point && !self.point_exists( p ) {
                            self.space.insert(p, 0);
                        }
                    }
                }
            }
        }
    }

    pub fn insert(&mut self, point: Coordinate4d, value: i32) -> Option<i32> {
        self.space.insert(point, value)
    }

    pub fn get_value(&self, point: Coordinate4d) -> Option<&i32> {
        self.space.get(&point)
    }

    pub fn sum_around_point(&self, point: Coordinate4d) -> i32 {
        let mut total = 0;
        for w in point.0-1..=point.0+1 {
            for z in point.3-1..=point.3+1 {
                for y in point.2-1..=point.2+1 {
                    for x in point.1-1..=point.1+1 {
                        let p: Coordinate4d = (w, x, y, z);
                        if p != point {
                            if let Some(v) = self.get_value(p) {
                                total += v;                                                        
                            }
                        }
                    }
                }
            }
        }
        total
    }

    pub fn print(&self) {
        for (p, v) in self.space.iter() {
            println!("({}, {}, {}, {}) => {}", p.0, p.1, p.2, p.3, v);
        }
    }

    pub fn points(&self) -> &HashMap<Coordinate4d, i32> {
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