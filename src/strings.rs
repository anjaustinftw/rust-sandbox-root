/*
    String Variants [Primitive str, String]
    str = immutable, fixed-length string stored in memory
    String = flexible, heap-allocated data structure
*/

pub fn run() {

    // str data type inferred
    let str_hello = "Hello";

    // String data type explicit
    let string_hello = String::from("hello!");

    println!("{}, {} {strl} by {stringl}.", str_hello, string_hello, strl = str_hello.len(), stringl = string_hello.len());

}
