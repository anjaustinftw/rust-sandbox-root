// Printing and formatting output to the console

pub fn run() {

    // Print to console
    println!("Hello world, from the print.rs file!");

    // Basic Formatting
    println!("{} is from {}.", "Aaron", "Colorado");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}.", "Aaron", "Colorado", "code");

    // Named Arguments
    println!("{name} likes to play {activity}.", name = "Aaron", activity = "EVE");

    // Placeholder Traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholer for debug trait
    println!("{:?}", (12, true, "Hello") );

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);

}
