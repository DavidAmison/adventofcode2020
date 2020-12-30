
#[derive(Debug, Clone)]
pub struct CircularListValue {
    value: u32,
    next_index: Option<usize>,
    index: usize,
    prev_index: Option<usize>,
}

impl CircularListValue {
    pub fn next(&self) -> Option<usize> {
        self.next_index
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct CircularList {
    pub values: Vec<CircularListValue>,
    pub pointer: Option<usize>,
}

impl CircularList {
    pub fn new() -> Self {
        CircularList {
            values: Vec::new(),
            pointer: None,
        }
    }

    /// Create a circular list from a vector
    pub fn from_vec(values: Vec<u32>) -> Self {
        let mut list = Vec::new();
        for (i, value) in values.iter().enumerate() {
            if i == 0 {
                // First Value
                list.push( CircularListValue{
                    value: value.clone(),
                    next_index: Some(i+1),
                    index: i,
                    prev_index: Some(values.len() - 1),
                })
            } else if i < (values.len() - 1) {
                // Middle values
                list.push( CircularListValue{
                    value: value.clone(),
                    next_index: Some(i+1),
                    index: i,
                    prev_index: Some(i-1),
                })
            } else {
                // Last Value
                list.push( CircularListValue{
                    value: value.clone(),
                    next_index: Some(0),
                    index: i,
                    prev_index: Some(i-1),
                })
            }
        }
        // list.last_mut().unwrap().next_index = None;
        CircularList {
            values: list,
            pointer: Some(0),
        }
    }

    pub fn insert_after(&mut self, value: u32, index: usize) {
        self.values.push( CircularListValue{ 
            value: value,
            next_index: self.values[index].next(),
            index: self.values.len() - 1,
            prev_index: Some(index),
        });
        self.values[index].next_index = Some(self.values.len() - 1);
    }

    pub fn move_after(&mut self, origin: usize, destination: usize) {
        // Moving round a lot of indexes which is unsafe rust but should be safe...
        unsafe {
            let o = self.get_mut(origin).unwrap() as *mut CircularListValue;
            let d = self.get_mut(destination).unwrap() as *mut CircularListValue;
            // Change references in node before origin
            let o_p;
            if let Some(i) = (*o).prev_index {
                o_p = &mut self.values[i] as *mut CircularListValue;
                (*o_p).next_index = (*o).next_index;
            }
            // Change references in node after origin
            let o_n;
            if let Some(i) = (*o).next_index {
                o_n = &mut self.values[i] as *mut CircularListValue;
                (*o_n).prev_index = (*o).prev_index;
            }
            
            // Change references in node after destination
            let d_n;
            if let Some(i) = (*d).next_index {
                d_n = &mut self.values[i] as *mut CircularListValue;
                (*d_n).prev_index = Some((*o).index);
            }            

            // Change references in origin
            (*o).prev_index = Some((*d).index);
            (*o).next_index = (*d).next_index;
            
            // Change references in destination
            (*d).next_index = Some((*o).index);
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut CircularListValue> {
        if self.pointer.is_none() {
            return None
        }

        let mut pointer = self.pointer.unwrap();
        for _ in 0..index {
            if let Some(p) = self.values[pointer].next_index {
                pointer = p;
            } else {
                return None
            }
        }
        self.values.get_mut(pointer)
    }

    pub fn get(&self, index: usize) -> Option<&CircularListValue> {
        if self.pointer.is_none() {
            return None
        }

        let mut pointer = self.pointer.unwrap();
        for _ in 0..index {
            if let Some(p) = self.values[pointer].next_index {
                pointer = p;
            } else {
                return None
            }
        }
        self.values.get(pointer)
    }

    pub fn increment_pointer(&mut self) {
        if let Some(i) = self.pointer {
            self.pointer = self.values[i].next_index;
        }
    }

    pub fn print(&self) {
        if self.pointer.is_none() {
            return;
        }

        let mut pointer = self.pointer.unwrap();
        print!("CircularList [{}", self.values[pointer].value());
        while let Some(p) = self.values[pointer].next() {
            if p == self.pointer.unwrap() {
                break
            }
            print!(", {}", self.values[p].value());
            pointer = p;

        }
        print!("]\n");
    }
}

pub struct CircularListPointer {
    index: Option<usize>,
    counter: usize,
}

impl CircularListPointer {
    pub fn new(index: usize) -> Self {
        CircularListPointer{
            index: Some(index),
            counter: 0,
        }
    }

    pub fn next(&mut self, list: &CircularList) {
        if let Some(i) = self.index {
            self.index = list.values[i].next_index;
            self.counter += 1;
        }
    }

    pub fn value(&self, list: &CircularList) -> Option<u32> {
        if let Some(i) = self.index {
            Some(list.values[i].value)
        } else {
            None
        }
    }

    pub fn index(&self, list: &CircularList) -> usize {
        self.counter
    }
}
