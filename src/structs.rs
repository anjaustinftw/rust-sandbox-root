/*
    Structs are used to create custom data types.

*/

struct Color {                                              // Traditional Struct
    red: u8,
    green: u8,
    blue: u8
    
}

struct TColor (u8,u8,u8);                                   // Tuple Struct

struct Person {

    first_name: String,
    last_name: String

}

impl Person {
    
    fn new(first: &str, last: &str) -> Person {             // Construct person
    
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }

    }

    fn full_name(&self) -> String {                         // get full name
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {               // set last name
        self.last_name = last.to_string()
    }

    fn to_tuple(self) -> (String, String) {                 // name to tuple

        (self.first_name, self.last_name)

    }
}

pub fn run() {

    let mut colubk = Color {

        red: 255,
        green: 0,
        blue: 0

    };

    colubk.red = 200;
    println!("Color: {} {} {}", colubk.red, colubk.green, colubk.blue);

    let mut tcolubk = TColor (255,0,0);
    tcolubk.0 = 200;
    println!("TColor: {} {} {}", tcolubk.0, tcolubk.1, tcolubk.2);

    let p = Person::new("John", "Doe");
    print!("Person {} {}\n", p.first_name, p.last_name);

    let mut fp = Person::new("Jane", "Doe");
    println!("Person {}", fp.full_name());

    fp.set_last_name("Williams");
    println!("Person {}", fp.full_name());

    println!("Person Tuple {:?}", fp.to_tuple());

}
