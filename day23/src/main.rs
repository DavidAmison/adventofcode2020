// mod linked_list;
// use linked_list::LinkedList;

mod circular_list;
use circular_list::{CircularList, CircularListPointer};

fn main() {
    part1();
    part2();
    println!("Part 2 complete");
}

fn part1() {
    let mut cups = CircularList::from_vec(vec!(3,9,4,6,1,8,5,2,7));
    // cups.print();
    
    for _ in 0..100 {
        let mut insert_positon = CircularListPointer::new(cups.pointer.unwrap());
        let current = cups.get(0).unwrap().value();
        let pickup1 = cups.get(1).unwrap().value();
        let pickup2 = cups.get(2).unwrap().value();
        let pickup3 = cups.get(3).unwrap().value();
        // Find the value we should be moving everything to
        let mut destination = circular_subtract(current, 9);
        while destination == current || destination == pickup1 || destination == pickup2 || destination == pickup3 {
            destination = circular_subtract(destination, 9);
        }
        // Move the insert_position pointer to the destination
        while insert_positon.value(&cups).unwrap() != destination {
            insert_positon.next(&cups);
        }
        let d = insert_positon.index(&cups);
        // Move the picked-up cups
        cups.move_after(1, d);
        cups.move_after(1, d);
        cups.move_after(1, d);
        
        cups.increment_pointer();
    }
    // cups.print();

    while cups.get(0).unwrap().value() != 1 {
        cups.increment_pointer();
    }
    cups.print();

    // println!("");
    // loop {
    //     cups.increment_pointer();
    //     if let Some(c) = cups.get(0) {
    //         if c.value() == 1 {
    //             break;
    //         } else {
    //             print!("{}", c.value());
    //         }
    //     }
    // }
    // println!("");

}

fn part2() {
    let mut cups_vector: Vec<u32> = vec!(3,9,4,6,1,8,5,2,7);
    for i in 10..=1000000 {
        cups_vector.push(i);
    }

    let mut cups = CircularList::from_vec(cups_vector);
    // cups.print();
    
    for _ in 0..1000 {
        let mut insert_positon = CircularListPointer::new(cups.pointer.unwrap());
        let current = cups.get(0).unwrap().value();
        let pickup1 = cups.get(1).unwrap().value();
        let pickup2 = cups.get(2).unwrap().value();
        let pickup3 = cups.get(3).unwrap().value();
        // Find the value we should be moving everything to
        let mut destination = circular_subtract(current, 9);
        while destination == current || destination == pickup1 || destination == pickup2 || destination == pickup3 {
            destination = circular_subtract(destination, 9);
        }
        // Move the insert_position pointer to the destination
        while insert_positon.value(&cups).unwrap() != destination {
            insert_positon.next(&cups);
        }
        let d = insert_positon.index(&cups);
        // Move the picked-up cups
        cups.move_after(1, d);
        cups.move_after(1, d);
        cups.move_after(1, d);
        
        cups.increment_pointer();
    }
    // cups.print();
}

fn circular_subtract(v: u32, max: u32) -> u32 {
    if v == 1 {
        max
    } else {
        v-1
    }
}
