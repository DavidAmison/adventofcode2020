fn main() {
    let mut hand1 = vec!(30,42,25,7,29,1,16,50,11,40,4,41,3,12,8,20,32,38,31,2,44,28,33,18,10);
    // let mut hand1 = vec!(9,2,6,3,1);
    hand1.reverse();
    let mut hand2 = vec!(36,13,46,15,27,45,5,19,39,24,14,9,17,22,37,47,43,21,6,35,23,48,34,26,49);
    // let mut hand2 = vec!(5,8,4,7,10);
    hand2.reverse();

    while hand1.len() > 0 && hand2.len() > 0 {
        if let (Some(p1), Some(p2)) = (hand1.pop(), hand2.pop()) {
            println!("P1 ==> {} vs {} <== P2 ", p1, p2);
            if p1 > p2 {
                hand1.insert(0, p1);
                hand1.insert(0, p2);
            } else if p2 > p1 {
                hand2.insert(0, p2);
                hand2.insert(0, p1);
            }
        }
    }
    println!("Hand 1: {:?}", hand1);
    println!("Hand 2: {:?}", hand2);

    let winning_hand;
    if hand1.len() > 0 {
        winning_hand = hand1;
    } else {
        winning_hand = hand2;
    }

    let mut score = 0;
    for (i, c) in winning_hand.iter().enumerate() {
        score += (i+1) * c;
    }

    println!("Score: {}", score);
}
