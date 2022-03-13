// Traditional struct
struct Colour {
    r: u8,
    g: u8,
    b: u8
}

// Tuple struct
struct RGB(u8,u8,u8);

struct Student {
    fname: String,
    house: String,
    year: u8
}

impl Student {
    // Constuctor method
    fn new(fname: &str, house: &str, year:u8) -> Student {
        Student {
            fname: fname.to_string(),
            house: house.to_string(),
            year: year
        }
    }
    // Get details
    fn get_details(&self) -> String {
        format!("Name: {}, House: {}, Year: {}", self.fname, self.house, self.year)
    }
    // Update Year
    fn set_year(&mut self, year:u8) {
        self.year = year;
    }
    // Student in tuple form
    fn to_tuple(self) -> (String, String, u8) {
        // Return
        (self.fname, self.house, self.year)
    }
}


pub fn run() {
    // Colour
    let mut c = Colour {
        r: 234,
        g: 80,
        b: 72
    };
    c.b = 100;
    println!("RGB: {},{},{}",c.r,c.g,c.b);

    let mut rgb = RGB(172,23,255);
    println!("RGB: {},{},{}",rgb.0,rgb.1,rgb.2);

    // Student
    let mut s = Student::new("Hermione", "Gryffindor",6);
    println!("Hogwarts Student: {}", s.get_details());
    s.set_year(7);
    println!("Hogwarts Student: {}", s.get_details());
    println!("As tuple = {:?}", s.to_tuple());
}