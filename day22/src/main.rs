use std::iter::FromIterator;

fn main() {
    println!("--- Part 1 ---");
    // let mut hand1 = vec!(9,2,6,3,1);
    // let mut hand2 = vec!(5,8,4,7,10);
    let mut hand1 = vec!(30,42,25,7,29,1,16,50,11,40,4,41,3,12,8,20,32,38,31,2,44,28,33,18,10);
    let mut hand2 = vec!(36,13,46,15,27,45,5,19,39,24,14,9,17,22,37,47,43,21,6,35,23,48,34,26,49);
    hand1.reverse();
    hand2.reverse();

    let winner = play_game(&mut hand1,&mut hand2);
    println!("Game won by P{}", winner);

    // Calculate the score of the winner
    let winning_hand;
    if winner == 1 {
        winning_hand = hand1;
    } else {
        winning_hand = hand2;
    }

    let mut score = 0;
    for (i, c) in winning_hand.iter().enumerate() {
        score += (i+1) * c;
    }

    println!("Score: {}", score);

    println!("\n--- Part 2 ---");

    let mut hand1 = vec!(30,42,25,7,29,1,16,50,11,40,4,41,3,12,8,20,32,38,31,2,44,28,33,18,10);
    let mut hand2 = vec!(36,13,46,15,27,45,5,19,39,24,14,9,17,22,37,47,43,21,6,35,23,48,34,26,49);
    hand1.reverse();
    hand2.reverse();

    let winner = play_game_recurse(&mut hand1, &mut hand2);
    println!("Game won by P{}", winner);

    // Calculate the score of the winner
    let winning_hand;
    if winner == 1 {
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

/// Play the non-recursive game
/// 
/// # Arguments
/// 
/// * `hand1` player 1's starting hand
/// * `hand2` player 2's starting hand
/// 
/// # Returns
/// 
/// * the winner (1 or 2)
fn play_game(hand1: &mut Vec<usize>, hand2: &mut Vec<usize>) -> usize {
    while hand1.len() > 0 && hand2.len() > 0 {
        if let (Some(p1), Some(p2)) = (hand1.pop(), hand2.pop()) {
            // println!("P1 ==> {} vs {} <== P2 ", p1, p2);
            if p1 > p2 {
                hand1.insert(0, p1);
                hand1.insert(0, p2);
            } else if p2 > p1 {
                hand2.insert(0, p2);
                hand2.insert(0, p1);
            }
        }
    }
    if hand1.len() > 0 {
        1
    } else {
        2
    }
}

/// Play the recursive game
/// 
/// # Arguments
/// 
/// * `hand1` player 1's starting hand
/// * `hand2` player 2's starting hand
/// 
/// # Returns
/// 
/// * the winner (1 or 2)
fn play_game_recurse(h1: &mut Vec<usize>, h2: &mut Vec<usize>) -> usize {
    // println!("\n--- NEW GAME ---");

    let mut played_hands = Vec::new();
    
    while h1.len() > 0 && h2.len() > 0 {
        // Check if we have played this hand before
        let id = hand_id(&h1, &h2);
        if played_hands.contains(&id) {
            // println!("Repeated hand: {}", id);
            break // defaults to player 1 win
        }
        played_hands.push(id);

        if let (Some(p1), Some(p2)) = (h1.pop(), h2.pop()) {
            // println!("P1 ==> {} vs {} <== P2 ", p1, p2);
            let mut winner = 0;
            // println!("P1:{} P2:{} H1:{} H2:{}", p1, p2, h1.len(), h2.len());
            if p1 <= h1.len() && p2 <= h2.len() {
                // Recurse
                let l1 = h1.len();
                let l2 = h2.len();
                let mut h1t = Vec::from_iter(h1[l1-p1..].iter().cloned());
                let mut h2t = Vec::from_iter(h2[l2-p2..].iter().cloned());
                winner = play_game_recurse(&mut h1t, &mut h2t);
                // println!("--- END GAME ---\n");
                // println!("Recursive Round won by P{}", winner);
            } else if p1 > p2 {
                winner = 1;
                // println!("Round won by P1: {} > {}", p1, p2);
            } else if p2 > p1 {
                // println!("Round won by P2: {} > {}", p2, p1);
                winner = 2;
            }

            if winner == 1 {
                h1.insert(0, p1);
                h1.insert(0, p2);
            } else if winner == 2 {
                h2.insert(0, p2);
                h2.insert(0, p1);
            } else {
                panic!("Round won by no-one!!!");
            }
        }
    }
    if h1.len() > 0 {
        1
    } else {
        2
    }
}

/// Generate a unique string from a given set of hands
fn hand_id(hand1: &[usize], hand2: &[usize]) -> String {
    format!("P1{:?} P2{:?}", hand1, hand2)
}
