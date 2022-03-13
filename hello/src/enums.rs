enum Move {
    up,
    down,
    left,
    right
}

// Function takes enum value as argument
fn move_mario(m: Move) {
    match m {
        Move::up => println!("Mario moved up"),
        Move::down => println!("Mario moved down"),
        Move::left => println!("Mario moved left"),
        Move::right => println!("Mario moved right")
    }
}

pub fn run() {
    let mut mario = Move::up;
    move_mario(mario);
    mario = Move::down;
    move_mario(mario);
    mario = Move::left; 
    move_mario(mario);
    mario = Move::right; 
    move_mario(mario);
}