/*
    String Variants [Primitive str, String]
    str = immutable, fixed-length string stored in memory
    String = flexible, heap-allocated data structure
*/

pub fn run() {

    // str data type inferred
    let str_hello = "Hello, ";

    /*
        str_hello.push('W');
        Will result in: error[E0599]: no method named `push` found for reference `&str` in the current scope
    */ 

    // String data type explicit
    let mut string_hello = String::from("hello! "); // string_hello is set to `mut` to allow push and push_str to modify string_hello

    string_hello.push('\u{1F600}'); // push can only ad hoc one `char`.

    string_hello.push_str(" \u{1F600} \u{1F600} \u{1F600} \u{1F600} \u{1F600} \u{1F600} \u{1F600}"); // push_str can add `char` and `&str`

    println!("{}{}\n{strl}:{stringl}", str_hello, string_hello, strl = str_hello.len(), stringl = string_hello.len());

}
