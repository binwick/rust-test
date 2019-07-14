//struct Color {
//    red: u8,
//    green: u8,
//    blue: u8,
//}

//struct Color(u8, u8, u8);// doesn't need a name tuple struct

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    fn to_tuple(&self) -> (String, String) {
        (self.first_name.clone(), self.last_name.clone())
    }
}

pub fn run() {
//    let mut c = Color {
//        red: 255,
//        green: 0,
//        blue: 0,
//    };
//    println!("Color: {} {} {}", c.red, c.green, c.blue);
//    let mut c = Color(255, 0, 0);
//    println!("Color: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("john", "Doe");
    println!("Person: {} ", p.full_name());

    p.set_last_name("Wick");
    println!("Person: {} ", p.full_name());
    println!("to tuple: {:?}", p.to_tuple());
    println!("Person: {} ", p.full_name());
}