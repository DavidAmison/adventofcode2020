fn main() {
    let door_public_key = 12578151;
    let card_public_key = 5051300;

    let door_loop = find_loop_size(door_public_key);
    let card_loop = find_loop_size(card_public_key);
    println!("Door Loop: {}", door_loop);
    println!("Card Loop: {}", card_loop);

    println!("Door Encryption Key: {}", encryption_key(card_public_key, door_loop));
    println!("Card Encryption Key: {}", encryption_key(door_public_key, card_loop));
}

/// Find the loop size used to geenrate the public key
/// 
/// # Arguments
/// 
/// `public_key` the public key to decrypt
fn find_loop_size(public_key: u64) -> usize {
    let mut x = 1;
    let mut i = 0;
    while x != public_key {
        i += 1;
        x = (x * 7) % 20201227;        
    }
    i
}

/// Calculate the encryption key given the public key and loop size
/// 
/// # Arguments
/// 
/// * `public_key` the public key to transform
/// * `loop_size` the loop size to apply
fn encryption_key(public_key: u64, loop_size: usize) -> u64 {
    let mut encryption_key = 1;
    for _ in 0..loop_size {
        encryption_key = (encryption_key * public_key) % 20201227;
    }
    encryption_key
}
