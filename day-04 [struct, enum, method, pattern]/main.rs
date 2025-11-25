// struct [custom data type]
// like js object or python class

struct User {
    name: String,
    age: u8,
    active: bool,
}
fn stuck_check() {
    let user1 = User {
        name: String::from("Abdur Shobur"),
        age: 22,
        active: true,
    };

    println!("Name= {}", user1.name);
}

fn mutable_stuck() {
    let mut user2 = User {
        name: String::from("test usr"),
        age: 32,
        active: true,
    };

    user2.active = false;

    println!("{}", user2.active)
}

// stuck er method
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Enum
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Book {
    title: String,
    price: u32,
}

impl Book {
    fn discount(&self) -> u32 {
        let x = (self.price * 10) / 100;
        self.price - x
    }
}

enum Payment {
    Cash(u32),
    Card(String),
    Bkash(String, u32),
}

fn checkPaymentMethod(method: Payment) {
    match method {
        Payment::Cash(amount) => { println!("Cash {}", amount) }
        Payment::Card(number) => { println!("Card {}", number) }
        Payment::Bkash(number, amount) => { println!("Bkash {} {}", number, amount) }
    }
}

fn main() {
    let payment = checkPaymentMethod(Payment::Bkash(String::from("017XXXXXXX"), 5000));
    println!("{:?}", payment);
    // let book1 = Book {
    //     title: String::from("Learn Rust"),
    //     price: 500,
    // };

    // println!("Book price is: {}", book1.discount());
}
