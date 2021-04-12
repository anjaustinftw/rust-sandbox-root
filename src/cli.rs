/*


*/

use std::env;

pub fn run() {

    let args: Vec<String>   = env::args().collect();
    let greeting            = args[1].clone();
    let person              = args[2].clone();

    println!("Greeting: {}, {}.", greeting, person);

    if greeting == "Hello" {

        println!("Hi, {}!", person);
    
    } else if greeting == "Aloha" {
    
        println!("Mahalo, {}!", person);
    
    } else if greeting == "status" {
    
        println!("All is well in paradise, my friend.");
    
    } else {
    
        println!("Come again?");
    
    }

}
