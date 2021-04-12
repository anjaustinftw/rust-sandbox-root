/*
    Reference Pointers refer us to a resource stored in memory.

    We cannot point directly to the value of a non-primitive without moving its data to the new variable.

*/

pub fn run() {

    let arr1 = [1,2,3];                         // primitive array
    let arr2 = arr1;                            // arr2 can copy arr1 by pointing to it directly, by its assigment.

    println!("Values: {:?}", (arr1, arr2));

    let vec1 = arr1.to_vec();                   // non-primitive vector arrays move when directly assigned
    let vec2 = &vec1;                           // vec1 must become &vec1 as to reference the original vector witout moving it.

    println!("Vecs: {:?}", (&vec1, vec2));

}
