// Enums are types which have a few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Match (like switch/case)
    match m {
        Movement::Up => println!("U"),
        Movement::Down => println!("D"),
        Movement::Left => println!("L"),
        Movement::Right => println!("R"),
    }
}

pub fn run() {
    let m1 = Movement::Up;
    let m2 = Movement::Down;
    let m3 = Movement::Left;
    let m4 = Movement::Right;

    move_avatar(m1);
    move_avatar(m2);
    move_avatar(m3);
    move_avatar(m4);
}
