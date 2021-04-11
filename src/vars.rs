// Variables hold primitive data or references to data
// Variables are immutable by default
// Ris is a block-scoped language

pub fn run() {

    let name = "Aaron";
    let /*mut*/ age  = 37;  // `mut` allows us to change the value of
                            // `age` downstream of its original
                            // initialization. Its use here wil cause
                            // the compiler to throw up a warning,
                            // because the value of `age` is not
                            // modified after its original init.
                            // Yet, the code will compile and run. 

    println!("My name is {} and my age is {}.", name, age);

    // Defining CONSTANTS
    const ID: u32 = 001;
    println!("ID: {}.", ID);

    // Assigning multiple variables inline
    let ( my_name, my_age ) = ("Aaron",37);
    println!("{} is {}.", my_name, my_age);

}
 