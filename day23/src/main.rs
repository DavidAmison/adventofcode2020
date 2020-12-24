fn main() {
    part1();
    // part2();
}

fn part1() {
    let mut cups: Vec<u32> = vec!(3,9,4,6,1,8,5,2,7);
    // let mut cups: Vec<u32> = vec!(3,8,9,1,2,5,4,6,7);
    for _ in 0..100 {
        let current = cups.remove(0);
        let pickup1 = cups.remove(0);
        let pickup2 = cups.remove(0);
        let pickup3 = cups.remove(0);
        let mut destination = circular_subtract(current, 9);
        let i;
        loop {
            if let Some(ind) = index(&cups, destination) {
                i = ind;
                break;
            } else {
                destination = circular_subtract(destination, 9);
            }
        }
        cups.insert(i+1, pickup3);
        cups.insert(i+1, pickup2);
        cups.insert(i+1, pickup1);
        cups.push(current);
    }
    loop {
        let v = cups.remove(0);
        if v == 1 {
            break;
        } else {
            cups.push(v);
        }
    }
    println!("");
    for c in cups {
        print!("{}", c);
    }
    println!("");

}

fn part2() {
    let mut cups: Vec<u32> = vec!(3,9,4,6,1,8,5,2,7);
    for i in 10..1000001 {
        cups.push(i);
    }
    println!("Cups populated");
    println!("{}", cups.iter().max().unwrap());
    // let mut cups: Vec<u32> = vec!(3,8,9,1,2,5,4,6,7);
    for _ in 0..10000 {
        let mut next = Vec::new(); 
        let current = cups.remove(0);
        let pickup1 = cups.remove(0);
        let pickup2 = cups.remove(0);
        let pickup3 = cups.remove(0);
        let mut destination = circular_subtract(current, 1000000);
        while destination == current || destination == pickup1 || destination == pickup2 || destination == pickup3 {
            destination = circular_subtract(destination, 1000000);
        }
        // println!("Destination: {}", destination);
        for cup in &cups {
            next.push(*cup);
            if cup == &destination {
                // Insert the picked-up cooks
                // println!("Inserting picked-up cups {},{},{} after {}", pickup1, pickup2, pickup3, cup);
                next.push(pickup1);
                next.push(pickup2);
                next.push(pickup3);
            }
        }
        next.push(current);
        cups = next;
        // let i = index(&cups, destination).unwrap();
        // println!("index: {}, cups: {}", i, cups.len());
        // cups.insert(i+1, pickup3);
        // cups.insert(i+1, pickup2);
        // cups.insert(i+1, pickup1);
        // cups.push(current);
    }
    
    // while let cup = cups.remove(0) {
    //     if cup == 1 {
    //         break
    //     }
    // }

    // println!("{:?}", cups.remove(0));
    // println!("{:?}", cups.remove(0));
}

fn index(vector: &Vec<u32>, value: u32) -> Option<usize> {
    println!("Searching for {}", value);
    for i in 0..vector.len() {
        if value == vector[i] {
            return Some(i)
        }
    }
    return None
}

fn circular_subtract(v: u32, max: u32) -> u32 {
    if v == 1 {
        max
    } else {
        v-1
    }
}
