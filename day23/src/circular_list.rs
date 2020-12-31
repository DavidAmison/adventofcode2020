
#[derive(Debug, Clone)]
pub struct CircularListValue {
    value: u32,
    next: usize,
}

impl CircularListValue {
    /// Return the index of the next value
    pub fn next(&self) -> usize {
        self.next
    }

    /// Return the value
    pub fn value(&self) -> u32 {
        self.value
    }

    /// Set the index of the next value
    pub fn set_next(&mut self, next: usize) {
        self.next = next;
    }
}

#[derive(Debug, Clone)]
pub struct CircularList {
    pub values: Vec<CircularListValue>,
    pub pointer: Option<usize>,
}

impl CircularList {
    /// Create a circular list from a vector
    pub fn from_vec(values: Vec<u32>) -> Self {
        let mut list = Vec::new();
        for (i, value) in values.iter().enumerate() {
            if i < (values.len() - 1) {
                // Middle values
                list.push( CircularListValue{
                    value: value.clone(),
                    next: i+1,
                })
            } else {
                // Last Value
                list.push( CircularListValue{
                    value: value.clone(),
                    next: 0,
                })
            }
        }
        CircularList {
            values: list,
            pointer: Some(0),
        }
    }

    /// Get the value at the given index (starting at the current pointer)
    /// 
    /// # Arguments
    /// 
    /// * `index` the index to return
    /// 
    /// # Returns
    /// 
    /// * the CircularListValue at the given index
    pub fn get(&self, index: usize) -> Option<&CircularListValue> {
        if self.pointer.is_none() {
            return None
        }

        let mut pointer = self.pointer.unwrap();
        for _ in 0..index {
            pointer = self.values[pointer].next();
        }
        self.values.get(pointer)
    }

    /// Get a vector of the values between the given indeces (inclusive)
    /// 
    /// # Arguments
    /// 
    /// * `start` the start index of the slice
    /// * `end` the end index of the slice
    /// 
    /// # Returns
    /// * A vector of the values between the start and end indeces
    pub fn get_slice(&self, start: usize, end: usize) -> Option<Vec<u32>> {
        if self.pointer.is_none() {
            return None
        }

        let mut values = Vec::new();
        let mut pointer = self.pointer.unwrap();
        for i in 0..=end {
            if i >= start && i <= end {
                values.push(self.values[pointer].value());
            }
            pointer = self.values[pointer].next();
        }
        Some(values)
    }

    /// Move the pointer one place clockwise around the curcluar list
    pub fn increment_pointer(&mut self) {
        if let Some(i) = self.pointer {
            self.pointer = Some(self.values[i].next);
        }
    }

    // Print all elements in the curcular list - starting from the current pointer
    pub fn print(&self) {
        if self.pointer.is_none() {
            return;
        }

        let mut pointer = self.pointer.unwrap();
        print!("CircularList [{}", self.values[pointer].value());
        loop {
            let p = self.values[pointer].next();
            if p == self.pointer.unwrap() {
                break
            }
            print!(", {}", self.values[p].value());
            pointer = p;
        }
        print!("]\n");
    }

    /// Print the first n elements of the circular list - starting from the current pointer
    pub fn printn(&self, n: usize) {
        if self.pointer.is_none() {
            return;
        }

        let mut pointer = self.pointer.unwrap();
        print!("CircularList [{}", self.values[pointer].value());
        for _ in 0..n {
            let p = self.values[pointer].next();
            print!(", {}", self.values[p].value());
            pointer = p;
        }
        print!(" ... ]\n");
    }
}
