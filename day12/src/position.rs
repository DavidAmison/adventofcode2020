use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Direction<T> {
    pub dir: T,
    pub val: i32,
}

impl <T>Direction<T>
where
    T: Copy,
    T: Clone,
{
    pub fn new(dir: T, val: i32) -> Self {
        Direction::<T>{
            dir,
            val,
        }
    }
}

impl <T>Add for Direction<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Direction::<T>{
            dir: self.dir,
            val: self.val + other.val,
        }
    }
}

impl <T>Add<i32> for Direction<T> {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        Direction::<T>{
            dir: self.dir,
            val: self.val + other,
        }
    }
}

/// ZST (zero-size-type) for holding cardinal directions
#[derive(Debug, Copy, Clone)]
pub struct North;
#[derive(Debug, Copy, Clone)]
pub struct East;
#[derive(Debug, Copy, Clone)]
pub struct South;
#[derive(Debug, Copy, Clone)]
pub struct West;

/// Structure for storing a position in N, E co-ordinates
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct Position {
    pub N: Direction<North>,
    pub E: Direction<East>,
}

impl Position {
    /// Create a new position struct
    /// 
    /// # Arguments
    /// 
    /// 
    /// * `north` the north position
    /// * `south` the south position
    pub fn new(north: i32, east: i32) -> Self {
        Position {
            N: Direction::new(North, north),
            E: Direction::new(East, east),
        }
    }
}

impl Add<Self> for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Position{
            N: self.N + other.N,
            E: self.E + other.E,
        }
    }
}

impl Add<Direction<North>> for Position {
    type Output = Self;

    fn add(self, other: Direction<North>) -> Self {
        Position{
            N: self.N + other.val,
            E: self.E,
        }
    }
}

impl Add<Direction<East>> for Position {
    type Output = Self;

    fn add(self, other: Direction<East>) -> Self {
        Position{
            N: self.N,
            E: self.E + other.val,
        }
    }
}

impl Add<Direction<South>> for Position {
    type Output = Self;

    fn add(self, other: Direction<South>) -> Self {
        Position{
            N: self.N + (-other.val),
            E: self.E,
        }
    }
}

impl Add<Direction<West>> for Position {
    type Output = Self;

    fn add(self, other: Direction<West>) -> Self {
        Position{
            N: self.N,
            E: self.E + (-other.val),
        }
    }
}