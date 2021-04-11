/*
    Vectors are flexible arrays of matching data types.

*/

pub fn run() {

    let mut numbers: Vec<i32>;                                                          // Declaration of Vector array for i32 data types

    numbers = [1,2,3,4,5].to_vec();                                                     // Assign i32 values to Vector members
    numbers[2] = 22;                                                                    // Member modification, Null change in Vector length

    println!("{:?}", numbers);                                                          // Print Vector array

    numbers.push(6);                                                                    // Flexing Vector array length
    numbers.push(7);
    numbers.push(88);                                                                   // This member value never prints

    println!("The value in numbers[4] is {}.", numbers[4]);                             // Print single Vector array value

    println!("Array Length: {}", numbers.len());                                        // Get Vector array length

    println!("Array occupies {} Bytes in memory.", std::mem::size_of_val(&numbers));    // Get Vector array stack allocation

    numbers.pop();                                                                      // Pop out last value, flexing Vector array length

    let slice: &[i32] = &numbers[2..6];                                                 // Get Vectors members 0..3

    println!("Slice: {:?}", slice);                                                     // Print sliced Vector array member values

    numbers.push(99);                                                                   // Flexing Vector array length

    println!("{:?}", numbers);                                                          // Print Vector array

    for x in numbers.iter() {                                                           // Itterate through members

        println!("#: {}", x);

    }

    for x in numbers.iter_mut() {                                                       // Iterate through and modify members

        *x *= 2;

    }

    println!("Vector member: {:?}", numbers);

}
