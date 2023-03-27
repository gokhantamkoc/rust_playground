// Enums are types which have a few definite values

enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(_movement: Movement) {
    // Perform action depending on _movement

    match _movement {
        Movement::Up => println!("Avatar moves up!"),
        Movement::Down => println!("Avatar moves down!"),
        Movement::Left => println!("Avatar moves left!"),
        Movement::Right => println!("Avatar moves right!"),
    }
}

pub fn run() {
    let mut avatar1 = Movement::Left;
    move_avatar(avatar1);
    avatar1 = Movement::Left;
    move_avatar(avatar1);
    avatar1 = Movement::Left;
    move_avatar(avatar1);
    avatar1 = Movement::Up;
    move_avatar(avatar1);
}
