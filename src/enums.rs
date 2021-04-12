/*
    Enums allow you to define a type by enumerating its possible variants.

*/

enum Movement {
    // Variants (variations of behavior categorized as Movement)
    Up,
    Down,
    Right,
    Left
}

fn move_avatar(m: Movement) {
    // Input-directed behavior
    match m {
        Movement::Up => println!("Avatar moving Up.");
        Movement::Down => println!("Avatar moving Down.");
        Movement::Right => println!("Avatar moving Right.");
        Movement::Left => println!("Avatar moving Left.");
    }
}

pub fn run() {



}
