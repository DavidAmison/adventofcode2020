
#[derive(Debug, Clone)]
pub struct LinkedListValue {
    value: u32,
    next_index: Option<usize>,
    index: usize,
    prev_index: Option<usize>,
}

impl LinkedListValue {
    pub fn next(&self) -> Option<usize> {
        self.next_index
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct LinkedList {
    values: Vec<LinkedListValue>,
    pointer: Option<usize>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
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
                list.push( LinkedListValue{
                    value: value.clone(),
                    next_index: Some(i+1),
                    index: i,
                    prev_index: None,
                })
            } else if i < (values.len() - 1) {
                // Middle values
                list.push( LinkedListValue{
                    value: value.clone(),
                    next_index: Some(i+1),
                    index: i,
                    prev_index: Some(i-1),
                })
            } else {
                // Last Value
                list.push( LinkedListValue{
                    value: value.clone(),
                    next_index: None,
                    index: i,
                    prev_index: Some(i-1),
                })
            }
        }
        list.last_mut().unwrap().next_index = None;
        LinkedList {
            values: list,
            pointer: Some(0),
        }
    }

    pub fn insert_after(&mut self, value: u32, index: usize) {
        self.values.push( LinkedListValue{ 
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
            let o = self.get_mut(origin).unwrap() as *mut LinkedListValue;
            let d = self.get_mut(destination).unwrap() as *mut LinkedListValue;
            println!("o: {:?}", *o);
            println!("d: {:?}", *d);
            // Change references in node before origin
            let o_p;
            if let Some(i) = (*o).prev_index {
                o_p = &mut self.values[i] as *mut LinkedListValue;
                (*o_p).next_index = (*o).next_index;
            } else {
            }
            // Change references in node after origin
            let o_n;
            if let Some(i) = (*o).next_index {
                o_n = &mut self.values[i] as *mut LinkedListValue;
                (*o_n).prev_index = (*o).prev_index;
            }
            
            // Change references in node after destination
            let d_n;
            if let Some(i) = (*d).next_index {
                d_n = &mut self.values[i] as *mut LinkedListValue;
                (*d_n).prev_index = Some((*o).index);
            }            

            // Change references in origin
            (*o).prev_index = Some((*d).index);
            (*o).next_index = (*d).next_index;
            
            // Change references in destination
            (*d).next_index = Some((*o).index);

            println!("o: {:?}", *o);
            println!("d: {:?}", *d);
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut LinkedListValue> {
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

    pub fn get(&self, index: usize) -> Option<&LinkedListValue> {
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

    pub fn print(&self) {
        if self.pointer.is_none() {
            return;
        }

        let mut pointer = self.pointer.unwrap();
        print!("LinkedList: [{}", self.values[pointer].value());
        while let Some(p) = self.values[pointer].next() {
            print!(", {}", self.values[p].value());
            pointer = p;
        }
        print!("]\n");
    }
}
