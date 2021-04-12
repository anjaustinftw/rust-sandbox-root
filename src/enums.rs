/*
    Enums allow you to define a type by enumerating its possible variants.

*/

enum Movement {                                                     // Variants (variations of behavior categorized as Movement)
    
    Up,
    Down,
    Right,
    Left
    
}

fn move_avatar(m: Movement) {
    
    match m {                                                       // Input-directed behavior options
        Movement::Up    => println!("Avatar moving Up."),
        Movement::Down  => println!("Avatar moving Down."),
        Movement::Right => println!("Avatar moving Right."),
        Movement::Left  => println!("Avatar moving Left.")
    }

}

pub fn run() {

    let avtr_up     = Movement::Up;
    let avtr_down   = Movement::Down;
    let avtr_right  = Movement::Right;
    let avtr_left   = Movement::Left;

    move_avatar(avtr_up);                                           // Directional inputs triggered by signals from user interface
    move_avatar(avtr_down);
    move_avatar(avtr_right);
    move_avatar(avtr_left);

}
