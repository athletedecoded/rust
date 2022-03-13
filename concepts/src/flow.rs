/*
* if and else "arms" must return the same type
* all cases must be handled by match arms
*/
#[derive(Debug)]
enum Sport {
    Bball,
    Fball { goals: i32, wins: i32 },
    F1(String),
    Swimming(i32, i32, i32),
    Cycling
}
// Define enum method
impl Sport {
    fn play(&self) {
        println!("{:?}", self);
    }
}

pub fn run() {
    // let-if
    let pick_prime = true;
    let number = if pick_prime { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let y = 11;
    let is_odd = if y%2 ==0 {false} else {true};
    println!("Is {} odd: {}", y, is_odd);

    // for
    for number in (1..=5).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    
    // enum methods
    let f1 = Sport::F1(String::from("Mclaren"));
    f1.play();
    let football = Sport::Fball{goals: 5, wins:3};
    football.play();

    // match
    println!("{}",ticket_price(f1));
    println!("{}",ticket_price(Sport::F1(String::from("Mercedes"))));
    println!("{}",ticket_price(Sport::Bball));
    println!("{}",ticket_price(Sport::Cycling));
    println!("{}",ticket_price(Sport::Swimming(3,12,6)));
    println!("{}",ticket_price(football));

    // matching + binding
    let num = 28;
    match num {
        n @ 0..=17 => println!("{:?} year olds drink fizzy drink",n),
        n @ 18..=21 => println!("{:?} year olds drink whatever is cheap", n),
        n => println!("{:?} year olds drink whatever they want!", n)
    }

}

fn ticket_price(sport: Sport) -> u8 {
    print!("The ticket price for {:?} is: $", sport);
    match sport {
        Sport::F1(team) => {
            // if condition = "guard"
            if team == "Mercedes" {
                100
            } else {
                75
            }
        },
        Sport::Fball{goals, wins} => {
            if wins > goals {
                25
            } else {
                50
            }
        },
        // Pipes = options
        Sport::Bball | Sport::Cycling => 30,
        // Catch all remaining cases
        _ => 10
    }
}



