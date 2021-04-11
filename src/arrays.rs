/*
    In Rust, arrays are fixed lists of matching data types.
*/

pub fn run() {

    let numbers: [i32; 5];                                                              // Declaration of Array for i32 data types with a length of 5, [0..4]

    numbers = [1,2,3,4,5];                                                              // Assign i32 values to Array

    println!("{:?}", numbers);                                                          // Print array

    println!("The value in numbers[4] is {}.", numbers[4]);                             // Print single array value

    println!("Array Length: {}", numbers.len());                                        // Get array length

    println!("Array occupies {} Bytes in memory.", std::mem::size_of_val(&numbers));    // Stack allocation

    let slice: &[i32] = &numbers[0..3];                                                 // Get array members 0..3

    println!("Slice: {:?}", slice);                                                     // Print sliced array member values

}
