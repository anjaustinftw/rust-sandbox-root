/*
    Tuples group values of different types.
    Limitation: Max of 12 elements
*/

pub fn run() {
    let person: (&str, &str, i8) = ("Aaron", "Colorado", 48);

    println!("{} is from {} and his age is {}.", person.0, person.1, person.2);
    
}
