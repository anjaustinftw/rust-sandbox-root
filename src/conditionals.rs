/*
    Conditionals validate relationships and act accordingly.

*/

pub fn run() {

    let cardinal:           Vec<char>   = ['N','S','E','W'].to_vec();   // Declare Vector array and assign values for `cardinal` directions
    let degrees:            Vec<u32>    = [0,180,90,270].to_vec();      // Declare Vector array and assign values for radial degrees
    
    let mut heading:        u32;                                        // Variable declaration
    heading                             = 180;                          // Variable initialization
    let docked:             bool        = false;
    
    let going_up:           char        = cardinal[0];                  // Up is relative
    let tacking:            u32         = degrees[0];                   // Use your compass
    let direction_in_life:  char        = 'N';                          // ['N','S','E','W']

    if direction_in_life == going_up && !docked {

        println!("Direction in Life = {}", direction_in_life);

        if heading == 180 {
            
            heading = tacking;
            println!("You're swingin' back around, to {there}*!", there=heading);

        }
        else if heading == degrees[0] {

            println!("You're tackin' {there}*.", there=heading);

        }

    }
    else if docked {

        println!("Get your boat on the water and out of the harbor!");

    }
    else {

        println!("Life is currently headed {where}", where=direction_in_life);

    }

// Ternary absense
let right_now = if direction_in_life == 'N' && tacking == 0 {true} else {false};

println!("I am currently headed North and tacking 0*: {:?}.", right_now);

}
