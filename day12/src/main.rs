/**
 * --- Day 12: Rain Risk ---
 * 
 * Your ferry made decent progress toward the island, but the storm came in faster than anyone expected. The ferry needs to take evasive actions!
 * Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route directly to safety, it produced extremely circuitous instructions. When the captain uses the PA system to ask if anyone can help, you quickly volunteer.
 * The navigation instructions (your puzzle input) consists of a sequence of single-character actions paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:
 * 
 *     Action N means to move north by the given value.
 *     Action S means to move south by the given value.
 *     Action E means to move east by the given value.
 *     Action W means to move west by the given value.
 *     Action L means to turn left the given number of degrees.
 *     Action R means to turn right the given number of degrees.
 *     Action F means to move forward by the given value in the direction the ship is currently facing.
 * 
 * The ship starts by facing east. Only the L and R actions change the direction the ship is facing. (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units, but would still move east if the following action were F.)
 * For example:
 * 
 * F10
 * N3
 * F7
 * R90
 * F11
 * 
 * These instructions would be handled as follows:
 * 
 *     F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
 *     N3 would move the ship 3 units north to east 10, north 3.
 *     F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
 *     R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
 *     F11 would move the ship 11 units south to east 17, south 8.
 * 
 * At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.
 * Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?
 * 
 *  --- Part Two ---
 * 
 * Before you can give the destination to the captain, you realize that the actual action meanings were printed on the back of the instructions the whole time.
 * Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:
 * 
 *     Action N means to move the waypoint north by the given value.
 *     Action S means to move the waypoint south by the given value.
 *     Action E means to move the waypoint east by the given value.
 *     Action W means to move the waypoint west by the given value.
 *     Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
 *     Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
 *     Action F means to move forward to the waypoint a number of times equal to the given value.
 * 
 * The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.
 * For example, using the same instructions as above:
 * 
 *     F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
 *     N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
 *     F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
 *     R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
 *     F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.
 * 
 * After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.
 * Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?
 * 
 */


use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Direction<T> {
    dir: T,
    val: i32,
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
struct North;
#[derive(Debug, Copy, Clone)]
struct East;
#[derive(Debug, Copy, Clone)]
struct South;
#[derive(Debug, Copy, Clone)]
struct West;

/// Structure for storing a position in N, E co-ordinates
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
struct Position {
    N: Direction<North>,
    E: Direction<East>,
}

impl Position {
    /// Create a new position struct
    /// 
    /// # Arguments
    /// 
    /// 
    /// * `north` the north position
    /// * `south` the south position
    fn new(north: i32, east: i32) -> Self {
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
struct Ship1 {
    direction: char,
    position: Position, 
}

impl Ship1 {
    /// Creates a new ship with the given direction and position
    /// 
    /// # Arguments
    /// 
    /// * `direction` the direction the ship is pointing (N, E, S or W)
    /// * `position` the position of the ship
    fn new(direction: char, position: Position) -> Self {
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
    fn execute_instruction(&mut self, instr: char, value: i32) {
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
struct Ship2 {
    ship_position: Position, 
    waypoint_position: Position,
}

impl Ship2 {
    /// Create a new ship with a given position and waypoint position
    /// 
    /// # Arguments
    /// 
    /// * `ship_position` the position of the ship
    /// * `waypoint_position` the position of the waypoint
    fn new(ship_position: Position, waypoint_position: Position ) -> Self {
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
    fn execute_instruction(&mut self, instr: char, value: i32) {
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


fn main() {
    let mut instructions = read_in_lines("src/directions.txt");

    let mut ship1 = Ship1::new('E', Position::new(0,0) );
    let mut ship2 = Ship2::new(Position::new(0,0), Position::new(1,10));

    for line in instructions.iter_mut() {
        // Insert a space so we can separate by whitespace
        line.insert(1, ' ');
        let temp: Vec<&str>  = line.split_whitespace().collect();
        // Extract the instruction
        let dir = temp[0].chars().nth(0).unwrap();
        let value = temp[1].parse::<i32>().unwrap();
        // Command the ship
        ship1.execute_instruction(dir, value);
        ship2.execute_instruction(dir, value);
        // println!("Moved {}{} -> new position {:?}", dir, value, ship);
    }
    println!("\n--- Part 1 ---\n");
    println!("Current position: {:#?}", ship1);
    println!("Manhatten distance: {}", ship1.position.E.val.abs() + ship1.position.N.val.abs());

    println!("\n--- Part 2 ---\n");
    println!("Current position: {:#?}", ship2);
    println!("Manhatten distance: {}", ship2.ship_position.E.val.abs() + ship2.ship_position.N.val.abs());
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
