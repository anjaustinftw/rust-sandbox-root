// Data Types in Rust : https://doc.rust-lang.org/book/ch03-02-data-types.html#:~:text=Rust%20has%20two%20primitive%20compound%20types%3A%20tuples%20and%20arrays.
// bool : The boolean type.
// char : A character type.
// int
// i8 : The 8-bit signed integer type.
// i16 : The 16-bit signed integer type.
// i32 : The 32-bit signed integer type.
// i64 : The 64-bit signed integer type.
// isize : The pointer-sized signed integer type.
// uint
// u8 : The 8-bit unsigned integer type.
// u16 : The 16-bit unsigned integer type.
// u32 : The 32-bit unsigned integer type.
// u64 : The 64-bit unsigned integer type.
// usize : The pointer-sized unsigned integer type.
// float
// f32 : The 32-bit floating point type.
// f64 : The 64-bit floating point type.
// Compound Data Types
// array : A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
// slice : A dynamically-sized view into a contiguous sequence, [T].
// str : String slices.
// tuple : A finite heterogeneous sequence, (T, U, ..).

// Rust is a statically typed language, but it does have the ability to infer types.
// Do the work for the compiler when you can.

pub fn run() {

    // Default infers "i32"
    let x = 1; // `_x` silences `#[warn(unused_variables)]` at `cargo run` when variable is not in use

    // Default infers "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 445445445445;
    let i32_min: i32 = std::i32::MIN;
    let i64_min: i64 = std::i64::MIN;
    let i32_max: i32 = std::i32::MAX;
    let i64_max: i64 = std::i64::MAX;

    // Find max size
    println!("Min i32: {}.", i32_min);
    println!("Min i64: {}.", i64_min);
    println!("Max i32: {}.", i32_max);
    println!("Max i64: {}.", i64_max);

    // Boolean
    let is_active = true; // inferred type boolean
    let _is_active: bool = true; // explicit type boolean

    println!("{:?}", (x, y, z, is_active, _is_active));

    print_type_of(&x);
    print_type_of(&y);
    print_type_of(&z);
    print_type_of(&is_active);
    print_type_of(&_is_active);

    // Function to print variable type
    // SOURCE: https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
    // 
    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

}
