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

    // Get byte capacity
    println!("Capacity: {} Bytes.", string_hello.capacity());

    // Check emptiness
    println!("String is empty: {}.", string_hello.is_empty());

    // Contains
    println!("Contains 'hello': {}", string_hello.contains("hello"));

    // Replace
    println!("Replace 'hello!' with 'there!': {str_greet}{string_greet}", str_greet = str_hello, string_greet = string_hello.replace("hello!", "there!"));

    // Loop on whitespace in string
    for word in string_hello.split_whitespace() {

        println!("{}", word);
    
    }

    let mut string_s = String::with_capacity(10);
    string_s.push('a');
    string_s.push('z');

    println!("{}", string_s);

    // Assertion testing
    assert_eq!(2, string_s.len());
    assert_eq!(10, string_s.capacity());

}
