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
    fn new(dir: T, val: i32) -> Self {
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

/// Structure which stores a ship with a position and pointing direction
/// Direction is stored as a char and must be one of N, E, S or W
#[derive(Debug)]
pub struct Ship1 {
    pub direction: char,
    pub position: Position, 
}

impl Ship1 {
    /// Creates a new ship with the given direction and position
    /// 
    /// # Arguments
    /// 
    /// * `direction` the direction the ship is pointing (N, E, S or W)
    /// * `position` the position of the ship
    pub fn new(direction: char, position: Position) -> Self {
        Ship1 {
            direction,
            position,
        }
    }

    /// Execute a ship instruction - modifying the ship position and direction
    /// 
    /// # Arguments
    /// 
    /// * `instr` the instruction to execute (N, E, S, W, L, R or F)
    /// * `value` the value associated with the instruction
    pub fn execute_instruction(&mut self, instr: char, value: i32) {
        match instr {
            'N' => self.position = self.position + Direction::new(North, value),
            'E' => self.position = self.position + Direction::new(East, value),
            'S' => self.position = self.position + Direction::new(South, value),
            'W' => self.position = self.position + Direction::new(West, value),
            'L' => self.rotate_left(value),
            'R' => self.rotate_right(value),
            'F' => self.execute_instruction(self.direction, value),
            _ => println!("Direction not recognised"),
        }
    }

    /// Rotate the ship direction anti-clockwise by the given angle.
    /// The angle must be 90, 180 or 270 (i.e. a whole number of right angles).
    /// 
    /// # Arguments
    /// 
    /// * `value` the angle to rotate (90, 180 or 270)
    fn rotate_left(&mut self, value: i32) {
        match ( self.direction, value) {
            ('N', 90) => self.direction = 'W',
            ('N', 180) => self.direction = 'S',
            ('N', 270) => self.direction = 'E',
            ('E', 90) => self.direction = 'N',
            ('E', 180) => self.direction = 'W',
            ('E', 270) => self.direction = 'S',
            ('S', 90) => self.direction = 'E',
            ('S', 180) => self.direction = 'N',
            ('S', 270) => self.direction = 'W',
            ('W', 90) => self.direction = 'S',
            ('W', 180) => self.direction = 'E',
            ('W', 270) => self.direction = 'N',
            _ => println!("L{} not recognised", value),     
        }
    }

    /// Rotate the ship direction clockwise by the given angle.
    /// The angle must be 90, 180 or 270 (i.e. a whole number of right angles)
    /// 
    /// # Arguments
    /// 
    /// * `value` the angle to rotate (90, 180 or 270)
    fn rotate_right(&mut self, value: i32) {
        match ( self.direction, value) {
            ('N', 90) => self.direction = 'E',
            ('N', 180) => self.direction = 'S',
            ('N', 270) => self.direction = 'W',
            ('E', 90) => self.direction = 'S',
            ('E', 180) => self.direction = 'W',
            ('E', 270) => self.direction = 'N',
            ('S', 90) => self.direction = 'W',
            ('S', 180) => self.direction = 'N',
            ('S', 270) => self.direction = 'E',
            ('W', 90) => self.direction = 'N',
            ('W', 180) => self.direction = 'E',
            ('W', 270) => self.direction = 'S',
            _ => println!("L{} not recognised", value),    
        }
    }
}

#[derive(Debug)]
pub struct Ship2 {
    pub ship_position: Position, 
    pub waypoint_position: Position,
}

impl Ship2 {
    /// Create a new ship with a given position and waypoint position
    /// 
    /// # Arguments
    /// 
    /// * `ship_position` the position of the ship
    /// * `waypoint_position` the position of the waypoint
    pub fn new(ship_position: Position, waypoint_position: Position ) -> Self {
        Ship2 {
            ship_position,
            waypoint_position,
        }
    }

    /// Execute a ship instruction - modifying the ship_position and waypoint_position
    /// 
    /// # Arguments
    /// 
    /// * `instr` the instruction to execute (N, E, S, W, L, R or F)
    /// * `value` the value associated with the instruction
    pub fn execute_instruction(&mut self, instr: char, value: i32) {
        match instr {
            'N' => self.waypoint_position = self.waypoint_position + Direction::new(North, value),
            'E' => self.waypoint_position = self.waypoint_position + Direction::new(East, value),
            'S' => self.waypoint_position = self.waypoint_position + Direction::new(South, value),
            'W' => self.waypoint_position = self.waypoint_position + Direction::new(West, value),
            'L' => self.rotate_left(value),
            'R' => self.rotate_right(value),
            'F' => {
                for _ in 0..value {
                    self.ship_position = self.ship_position + self.waypoint_position;
                }
            }
            _ => println!("Direction not recognised"),
        }
    }

    /// Rotate the waypoint position anti-clockwise by the given angle.
    /// The angle must be 90, 180 or 270 (i.e. a whole number of right angles)
    /// 
    /// # Arguments
    /// 
    /// * `value` the angle to rotate (90, 180 or 270)
    fn rotate_left(&mut self, value: i32) {
        match value {
            90 => {
                let n = self.waypoint_position.E.val;
                let e = -self.waypoint_position.N.val;
                self.waypoint_position = Position::new(n, e);
                }
            180 => {
                let n = -self.waypoint_position.N.val;
                let e = -self.waypoint_position.E.val;
                self.waypoint_position = Position::new(n, e);
            }
            270 => {
                let n = -self.waypoint_position.E.val;
                let e = self.waypoint_position.N.val;
                self.waypoint_position = Position::new(n, e);
            }
            _ => println!("Unrecognised left rotation: {}", value),
            }
    }

    /// Rotate the waypoint position clockwise by the given angle.
    /// The angle must be 90, 180 or 270 (i.e. a whole number of right angles)
    /// 
    /// # Arguments
    /// 
    /// * `value` the angle to rotate (90, 180 or 270)
    fn rotate_right(&mut self, value: i32) {
        match value {
            90 => {
                let n = -self.waypoint_position.E.val;
                let e = self.waypoint_position.N.val;
                self.waypoint_position = Position::new(n, e);
                }
            180 => {
                let n = -self.waypoint_position.N.val;
                let e = -self.waypoint_position.E.val;
                self.waypoint_position = Position::new(n, e);
            }
            270 => {
                let n = self.waypoint_position.E.val;
                let e = -self.waypoint_position.N.val;
                self.waypoint_position = Position::new(n, e);
            }
            _ => println!("Unrecognised right rotation: {}", value),
            }
    }
}