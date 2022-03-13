/*
`impl` --> implementation block which defines an associated function within the type scope 
`&self` --> function borrows the Self instance
*/
struct Rectangle {
    width: u32,
    height: u32
}

// Define struct method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn square(side: u32) -> Rectangle {
    Rectangle {
        width: side,
        height: side
    }
}

pub fn run() {
    let r1 = Rectangle {
        width: 10,
        height: 5
    };
    let r2 = Rectangle {
        width: 6,
        height: 2
    };
    let s1 = square(3);

    println!("R1 area: {}", r1.area());
    println!("R2 area: {}", r2.area());
    println!("S1 area: {}", s1.area());
    println!("R2 fit in R1? {}", r1.can_hold(&r2));
    println!("R1 fit in R2? {}", r2.can_hold(&r1));
    println!("S1 fit in R1? {}", r1.can_hold(&s1));
    println!("S1 fit in R2? {}", r2.can_hold(&s1));
}