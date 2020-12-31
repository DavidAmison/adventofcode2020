// mod linked_list;
// use linked_list::LinkedList;

mod circular_list;
use circular_list::CircularList;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("\n--- Part 1 ---");
    // By making the values sit in order in the vector we make indexing the vector much quicker!
    let mut cups = CircularList::from_vec(vec!(1,2,3,4,5,6,7,8,9));
    // Re-arrange the first elements to the correct order
    cups.values[2].set_next(8);
    cups.values[8].set_next(3);
    cups.values[3].set_next(5);
    cups.values[5].set_next(0);
    cups.values[0].set_next(7);
    cups.values[7].set_next(4);
    cups.values[4].set_next(1);
    cups.values[1].set_next(6);
    cups.values[6].set_next(2);
    cups.pointer = Some(2);
    cups.print();
    
    // Iterate 100 times
    for _ in 0..100 {
        let current = cups.get(0).unwrap().value();
        let pickup = cups.get_slice(1,3).unwrap();
        // Find the value we should be moving everything to
        let mut destination = circular_subtract(current, 9);
        while destination == current || destination == pickup[0] || destination == pickup[1] || destination == pickup[2] {
            destination = circular_subtract(destination, 9);
        }
        // Move the cups
        let i0 = pickup[0] as usize - 1;  // Index of the first picked-up cup
        let n2 = cups.values[pickup[2] as usize - 1].next();  // Index of the cup after the last picked-up cup
        let nd = cups.values[destination as usize - 1].next();  // Index of the cup after the destination cup
        
        cups.values[current as usize - 1].set_next(n2);  // Current cup now points to cup after the picked-up cups
        cups.values[destination as usize - 1].set_next(i0);  // Destination cup points to first picked-up cup
        cups.values[pickup[2] as usize - 1].set_next(nd);  // Last picked-up cup points to cup originally after destination cup
        
        cups.increment_pointer();
        // cups.print();
    }

    while cups.get(0).unwrap().value() != 1 {
        cups.increment_pointer();
    }
    cups.print();
    
    print!("Solution: ");
    loop {
        cups.increment_pointer();
        if let Some(c) = cups.get(0) {
            if c.value() == 1 {
                break;
            } else {
                print!("{}", c.value());
            }
        }
    }
    println!("");

}

fn part2() {
    println!("\n--- Part 2 ---");
    // Create a vector of all values from 1 to 1,000,000
    let cups_vector: Vec<u32> = (1..=1000000).collect();
    let mut cups = CircularList::from_vec(cups_vector);
    // Re-arrange the first elements to the correct order
    cups.values[2].set_next(8);
    cups.values[8].set_next(3);
    cups.values[3].set_next(5);
    cups.values[5].set_next(0);
    cups.values[0].set_next(7);
    cups.values[7].set_next(4);
    cups.values[4].set_next(1);
    cups.values[1].set_next(6);
    cups.values[6].set_next(9);
    cups.values[999999].set_next(2);
    cups.pointer = Some(2);
    cups.printn(15);
    
    // Iterate 10-million times
    for _ in 0..10000000 {
        let current = cups.get(0).unwrap().value();
        let pickup = cups.get_slice(1,3).unwrap();
        // Find the value we should be moving everything to
        let mut destination = circular_subtract(current, 1000000);
        while destination == current || destination == pickup[0] || destination == pickup[1] || destination == pickup[2] {
            destination = circular_subtract(destination, 1000000);
        }
        // Move the cups
        let i0 = pickup[0] as usize - 1;  // Index of the first picked-up cup
        let n2 = cups.values[pickup[2] as usize - 1].next();  // Index of the cup after the last picked-up cup
        let nd = cups.values[destination as usize - 1].next();  // Index of the cup after the destination cup
        
        cups.values[current as usize - 1].set_next(n2);  // Current cup now points to cup after the picked-up cups
        cups.values[destination as usize - 1].set_next(i0);  // Destination cup points to first picked-up cup
        cups.values[pickup[2] as usize - 1].set_next(nd);  // Last picked-up cup points to cup originally after destination cup
        
        // Move around the circle of cups
        cups.increment_pointer();
    }

    cups.pointer = Some(0);
    cups.printn(15);
    // Numbers too big for u32
    println!("Solution: {}", cups.get(1).unwrap().value() as u64 * cups.get(2).unwrap().value() as u64);
}

fn circular_subtract(v: u32, max: u32) -> u32 {
    if v == 1 {
        max
    } else {
        v-1
    }
}
