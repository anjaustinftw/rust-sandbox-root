/*
    It is possible to create infinite loops in Rust.

*/

pub fn run() {

    // let mut count = 0;

    // loop {
        
    //     count += 1;

    //     println!("No.: {}", count);

    //     if count == 20 {
    //         break;
    //     }

    // }

    // While Loop
    // while count <= 100 {

    //     if count % 15 == 0 {

    //         println!("FizzBuzz");

    //     } else if count % 3 == 0 {
            
    //         println!("Fizz");

    //     } else if count % 5 == 0 {
            
    //         println!("Buzz");

    //     } else {

    //         println!("{}", count);

    //     }

    //     count += 1;

    // }

    // For Range
    for x in 0..100 {

        if x % 15 == 0 {

            println!("FizzBuzz");

        } else if x % 3 == 0 {
            
            println!("Fizz");

        } else if x % 5 == 0 {
            
            println!("Buzz");

        } else {

            println!("{}", x);

        }

    }

}
