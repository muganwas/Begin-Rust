#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
    California,
    Colorado,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

impl Rectangle {
    fn square(s: u32) -> Rectangle {
        Self {
            width: s,
            height: s,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let _num = 3;
    let _num2 = Some(3);
    // let _num_sum = _num + _num2;
    // println!("option and integer sum: {}", _num_sum);
    let sq = Rectangle::square(30);
    let message = Message::Write(String::from("hello"));
    message.call();
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&sq));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Penny equivalent in cents: {}", value_in_cents(Coin::Penny));
    println!(
        "Quarter equivalent in cents: {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
    println!("Addition of 1 to 3: {:?}", plus_one(Some(3)));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
